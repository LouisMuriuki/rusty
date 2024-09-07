use std::ops::Deref;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smartpointer: {:#?}", self.data);
    }
}

impl Deref for CustomSmartPointer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn runcustomPointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
