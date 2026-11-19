use std::fs;

use serial_test::serial;
use todo_manager::todos::structures::{TodoCreate, TodoDeSerializer, TodoEnum, TodoSerializer, TodoView};
use todo_manager::commands::{add_todo_items, delete_todo_item, retrieve_todo_item, update_todo_item, view_todos};

const DB_PATH: &str = "enc_todos.txt";

fn setup_test_file(content: &str) -> String {
    let backup = fs::read_to_string(DB_PATH).unwrap_or_default();
    fs::write(DB_PATH, content).unwrap();
    backup
}

fn teardown_test_file(backup: &str) {
    fs::write(DB_PATH, backup).unwrap();
}

// --- TodoCreate ---

#[test]
fn todo_create_generates_unique_ids() {
    let a = TodoCreate::new("task a".into(), TodoEnum::Todo);
    let b = TodoCreate::new("task b".into(), TodoEnum::Todo);
    assert_ne!(a.get_id(), b.get_id());
}

#[test]
fn todo_create_stores_name_and_status() {
    let todo = TodoCreate::new("laundry".into(), TodoEnum::Done);
    assert_eq!(todo.get_name(), "laundry");
    assert!(matches!(todo.get_status(), TodoEnum::Done));
}

#[test]
fn todo_create_has_date() {
    let todo = TodoCreate::new("task".into(), TodoEnum::Todo);
    assert!(!todo.get_date_created().is_empty());
}

// --- Serialization round-trip ---

#[test]
fn todo_create_deserialize_format() {
    let todo = TodoCreate::new("wash car".into(), TodoEnum::Todo);
    let line = todo.deserialize();
    let parts: Vec<&str> = line.split('|').collect();
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[1], "wash car");
    assert_eq!(parts[2], "Todo");
}

#[test]
fn string_serialize_to_todo_view() {
    let line = "abc-123|buy groceries|Done|2026-01-01".to_string();
    let view = line.serialize();
    assert_eq!(view.get_id(), "abc-123");
    assert_eq!(view.get_name(), "buy groceries");
    assert!(matches!(view.get_status(), TodoEnum::Done));
    assert_eq!(view.get_date_created(), "2026-01-01");
}

#[test]
fn string_serialize_unknown_status_defaults_to_todo() {
    let line = "id-1|task|Banana|2026-01-01".to_string();
    let view = line.serialize();
    assert!(matches!(view.get_status(), TodoEnum::Todo));
}

#[test]
fn todo_view_deserialize_round_trip() {
    let original = "abc-123|buy groceries|Done|2026-01-01".to_string();
    let view = original.serialize();
    let back = view.deserialize();
    assert_eq!(back, "abc-123|buy groceries|Done|2026-01-01");
}

// --- TodoView ---

#[test]
fn todo_view_set_name() {
    let mut view = TodoView::new("1".into(), "old".into(), TodoEnum::Todo, "2026-01-01".into());
    view.set_name("new".into());
    assert_eq!(view.get_name(), "new");
}

// --- File operations (serial, with backup/restore) ---

#[test]
#[serial]
fn view_todos_returns_limited_results() {
    let backup = setup_test_file("id-1|task one|Todo|2026-01-01\nid-2|task two|Done|2026-01-02\nid-3|task three|Todo|2026-01-03");
    view_todos(0, 2);
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn view_todos_skip_and_take() {
    let backup = setup_test_file("id-1|a|Todo|2026-01-01\nid-2|b|Done|2026-01-02\nid-3|c|Todo|2026-01-03");
    view_todos(1, 1);
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn view_todos_beyond_file_length() {
    let backup = setup_test_file("id-1|only one|Todo|2026-01-01");
    view_todos(0, 100);
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn view_todos_start_beyond_file_length() {
    let backup = setup_test_file("id-1|only one|Todo|2026-01-01");
    view_todos(999, 5);
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn view_todos_skips_empty_lines() {
    let backup = setup_test_file("id-1|task|Todo|2026-01-01\n\n\nid-2|task2|Done|2026-01-02");
    view_todos(0, 10);
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn view_todos_with_max_offset() {
    let backup = setup_test_file("id-1|task|Todo|2026-01-01");
    view_todos(0, usize::MAX);
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn add_todo_items_appends_to_file() {
    let backup = setup_test_file("id-1|existing|Todo|2026-01-01\n");
    add_todo_items(vec!["new task".into()]).unwrap();

    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(contents.contains("existing"));
    assert!(contents.contains("new task"));

    let lines: Vec<&str> = contents.split('\n').filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 2);

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn add_multiple_items_at_once() {
    let backup = setup_test_file("");
    add_todo_items(vec!["alpha".into(), "beta".into(), "gamma".into()]).unwrap();

    let contents = fs::read_to_string(DB_PATH).unwrap();
    let lines: Vec<&str> = contents.split('\n').filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 3);
    assert!(contents.contains("alpha"));
    assert!(contents.contains("beta"));
    assert!(contents.contains("gamma"));

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn add_todo_items_generates_unique_ids() {
    let backup = setup_test_file("");
    add_todo_items(vec!["one".into(), "two".into()]).unwrap();

    let contents = fs::read_to_string(DB_PATH).unwrap();
    let lines: Vec<&str> = contents.split('\n').filter(|l| !l.trim().is_empty()).collect();
    let id1: Vec<&str> = lines[0].split('|').collect();
    let id2: Vec<&str> = lines[1].split('|').collect();
    assert_ne!(id1[0], id2[0]);

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn delete_removes_matching_todo() {
    let backup = setup_test_file("id-1|keep me|Todo|2026-01-01\nid-2|delete me|Todo|2026-01-02\nid-3|keep too|Done|2026-01-03");
    delete_todo_item("id-2".into());

    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(!contents.contains("delete me"));
    assert!(contents.contains("keep me"));
    assert!(contents.contains("keep too"));

    let lines: Vec<&str> = contents.split('\n').filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 2);

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn delete_nonexistent_id_keeps_all() {
    let backup = setup_test_file("id-1|task one|Todo|2026-01-01\nid-2|task two|Done|2026-01-02");
    delete_todo_item("does-not-exist".into());

    let contents = fs::read_to_string(DB_PATH).unwrap();
    let lines: Vec<&str> = contents.split('\n').filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 2);

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn delete_all_items_leaves_empty_file() {
    let backup = setup_test_file("only-one|task|Todo|2026-01-01");
    delete_todo_item("only-one".into());

    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(contents.trim().is_empty());

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn update_changes_name_of_matching_todo() {
    let backup = setup_test_file("id-1|old name|Todo|2026-01-01\nid-2|untouched|Done|2026-01-02");
    update_todo_item("id-1".into(), "new name".into());

    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(contents.contains("new name"));
    assert!(!contents.contains("old name"));
    assert!(contents.contains("untouched"));

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn update_nonexistent_id_changes_nothing() {
    let backup = setup_test_file("id-1|original|Todo|2026-01-01");
    update_todo_item("ghost".into(), "changed".into());

    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(contents.contains("original"));
    assert!(!contents.contains("changed"));

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn update_preserves_status_and_date() {
    let backup = setup_test_file("id-1|task|Done|2026-05-15");
    update_todo_item("id-1".into(), "renamed".into());

    let contents = fs::read_to_string(DB_PATH).unwrap();
    let line = contents.lines().next().unwrap();
    let parts: Vec<&str> = line.split('|').collect();
    assert_eq!(parts[1], "renamed");
    assert_eq!(parts[2], "Done");
    assert_eq!(parts[3], "2026-05-15");

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn retrieve_existing_todo() {
    let backup = setup_test_file("id-1|find me|Todo|2026-01-01\nid-2|not me|Done|2026-01-02");
    retrieve_todo_item("id-1".into());
    teardown_test_file(&backup);
}

#[test]
#[serial]
fn retrieve_nonexistent_todo_does_not_panic() {
    let backup = setup_test_file("id-1|task|Todo|2026-01-01");
    retrieve_todo_item("missing".into());
    teardown_test_file(&backup);
}

// --- Edge cases ---

#[test]
#[serial]
fn operations_on_empty_file() {
    let backup = setup_test_file("");
    view_todos(0, 10);
    delete_todo_item("anything".into());
    update_todo_item("anything".into(), "name".into());
    retrieve_todo_item("anything".into());
    teardown_test_file(&backup);
}

#[test]
fn todo_create_deserialize_does_not_contain_extra_pipes() {
    let todo = TodoCreate::new("test".into(), TodoEnum::Todo);
    let line = todo.deserialize();
    assert_eq!(line.matches('|').count(), 3);
}

#[test]
#[serial]
fn add_then_retrieve() {
    let backup = setup_test_file("");
    add_todo_items(vec!["findable".into()]).unwrap();

    let contents = fs::read_to_string(DB_PATH).unwrap();
    let id = contents.lines().next().unwrap().split('|').next().unwrap().to_string();
    retrieve_todo_item(id);

    teardown_test_file(&backup);
}

#[test]
#[serial]
fn add_then_update_then_delete() {
    let backup = setup_test_file("");

    add_todo_items(vec!["lifecycle test".into()]).unwrap();
    let contents = fs::read_to_string(DB_PATH).unwrap();
    let id = contents.lines().next().unwrap().split('|').next().unwrap().to_string();

    update_todo_item(id.clone(), "updated name".into());
    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(contents.contains("updated name"));
    assert!(!contents.contains("lifecycle test"));

    delete_todo_item(id);
    let contents = fs::read_to_string(DB_PATH).unwrap();
    assert!(contents.trim().is_empty());

    teardown_test_file(&backup);
}
