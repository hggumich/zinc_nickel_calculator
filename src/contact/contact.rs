struct Contact {
    company: String,
    website: String,
    name: String,
    title: String,
    phone: String,
    email: String,
    street_address: String,
    state: String,
    zip_code: i32,
}

pub fn contact() {

    let contact1 = Contact {
        company: String::from("Dispol of America"),
        website: ,
        name: String::from("Chris Kolar"),
        title: String::from("Technical Service Representative"),
        phone: String::from("734-261-0633"),
        email: String::from("ckolar@dipsolamerica.com"),
        street_address: String::from("34005 Schoolcraft St"),
        state: String::from("Mi"),
        zip_code: 48150,
    }


    let contact2 = Contact {
        company: String::from("Dispol of America"),
        website: ,
        name: String::from("John Szczypka"),
        title: String::from("NA Sales Manager"),
        phone: String::from("734-261-0633"),
        email: String::from("JSzczypka@dipsolamerica.com"),
        street_address: String::from("34005 Schoolcraft St"),
        state: String::from("Mi"),
        zip_code: 48150,
    }
    
    
    let contact3 = Contact {
        company: String::from("Brenntag Mid-South, Inc"),
        website: String::from("website: https://www.brenntagmid-south.com"),
        name: String::from("Tammy Hoang"),
        title: String::from("Account Manager - Durham District"),
        phone: String::from("270-832-5292"),
        email: String::from("ckolar@dipsolamerica.com"),
        street_address: String::from("34005 Schoolcraft St"),
        state: String::from("Mi"),
        zip_code: 48150,
    }


    println!("Tammy Hoang");
    println!("Title: Account Manager - Durham District");
    println!("email: thoang@brenntag.com");
    println!("phone: 270-832-5292");
    println!("Fischer Technology, Inc");
    println!("www.fischer-technology.com");
    println!("Bill Brecher");
    println!("Title: Sales Engineer");
    println!("email: bBrecher@fischer-technology.com");
    println!("phone: 860-249-6908");
}
