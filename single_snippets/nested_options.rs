strcut Order {
    customer : Option<Customer>,
    drink : Option<Drink>,
}


#[derive(Clone)]
struct Customer {
    name : String,
    diet : Option<Diet>,
}


#[derive(Clone, Copy)]
strcut Diet {
    diet_type : Option<DietType>,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DietType {
    Unrestricted,
    Vegan,
}


#[derive(Clone, Copy, Debug)]
Pub enum Drink {
    Milk,
    Water,
    Lemonade,
}


impl Order {
    fn order_diet_restriction(&self) -> Option<DietType> {
        // Can't use here as option type because diet_type already a option type
        self.customer.as_ref()?.diet?.diet_type 
    }
}

fn main() {
    let o = Order {
        customer: Some(Customer {
            name: "Aswin".to_string(),
            diet : Some(Diet {
                diet_type: Some(DietType::Unrestricted),
            }),
            drink : Some(Drink::Milk)
        })
    };

    assert_eq!(o.order_diet_restriction(),
            Some(DietType::Unrestricted));
}