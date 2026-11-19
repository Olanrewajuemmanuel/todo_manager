use uuid::Uuid;

pub trait TodoDeSerializer {
    fn deserialize(&self) -> String;
}

pub trait TodoSerializer {
    fn serialize(&self) -> TodoView;
}

#[derive(Debug, Clone)]
pub struct TodoView {
    id: String,
    name: String,
    status: TodoEnum,
    date_created: String,
}

#[derive(Debug, Clone)]
pub struct TodoCreate {
    id: String,
    name: String,
    status: TodoEnum,
    date_created: String,
}

impl TodoCreate {
    pub fn new(name: String, status: TodoEnum) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: id.to_string(),
            name,
            status,
            date_created: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_status(&self) -> &TodoEnum {
        &self.status
    }

    pub fn get_date_created(&self) -> &String {
        &self.date_created
    }
}

impl TodoDeSerializer for TodoCreate {
    fn deserialize(&self) -> String {
        format!(
            "{}|{}|{:?}|{}",
            self.get_id(),
            self.get_name(),
            self.get_status(),
            self.get_date_created()
        )
    }
}

impl TodoSerializer for String {
    fn serialize(&self) -> TodoView {
        let parts: Vec<&str> = self.split('|').collect();
        let status = match parts[2].trim() {
            "Done" => TodoEnum::Done,
            "Todo" => TodoEnum::Todo,
            _ => TodoEnum::Todo,
        };
        TodoView::new(
            parts[0].trim().to_string(),
            parts[1].trim().to_string(),
            status,
            parts[3].trim().to_string(),
        )
    }
}

impl TodoView {
    pub fn new(id: String, name: String, status: TodoEnum, date_created: String) -> Self {
        Self {
            id,
            name,
            status,
            date_created,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_status(&self) -> &TodoEnum {
        &self.status
    }

    pub fn get_date_created(&self) -> &String {
        &self.date_created
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl TodoDeSerializer for TodoView {
    fn deserialize(&self) -> String {
        format!(
            "{}|{}|{:?}|{}",
            self.id, self.name, self.status, self.date_created
        )
    }
}

#[derive(Debug, Clone)]
pub enum TodoEnum {
    Done,
    Todo,
}
