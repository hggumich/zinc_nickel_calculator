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

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Company: {}\nWebsite: {}\nName: {}\nTitle: {}\nPhone: {}\nEmail: {}\nStreet Address:{}\nState: {}\nZip Code: {}\n",
            self.company,
            self.website,
            self.name,
            self.title,
            self.phone,
            self.email,
            self.street_address,
            self.state,
            self.zip_code
        )
    }
}

pub fn contact() {
    let contact1 = Contact {
        company: String::from("Dispol of America"),
        website: String::from("https://www.dipsolamerica.com/"),
        name: String::from("Chris Kolar"),
        title: String::from("Technical Service Representative"),
        phone: String::from("734-261-0633"),
        email: String::from("ckolar@dipsolamerica.com"),
        street_address: String::from("34005 Schoolcraft St"),
        state: String::from("Mi"),
        zip_code: 48150,
    };

    let contact2 = Contact {
        company: String::from("Dispol of America"),
        website: String::from("https://www.dipsolamerica.com/"),
        name: String::from("John Szczypka"),
        title: String::from("NA Sales Manager"),
        phone: String::from("734-261-0633"),
        email: String::from("JSzczypka@dipsolamerica.com"),
        street_address: String::from("34005 Schoolcraft St"),
        state: String::from("Mi"),
        zip_code: 48150,
    };

    let contact3 = Contact {
        company: String::from("Brenntag Mid-South, Inc"),
        website: String::from("https://www.brenntagmid-south.com"),
        name: String::from("Tammy Hoang"),
        title: String::from("Account Manager - Durham District"),
        phone: String::from("270-832-5292"),
        email: String::from("thoang@brenntag.com"),
        street_address: String::from("11750 Fruehauf Dr."),
        state: String::from("NC"),
        zip_code: 28273,
    };

    let contact4 = Contact {
        company: String::from("Fischer Technology, Inc"),
        website: String::from("https://www.fischer-technology.com"),
        name: String::from("Bill Brecher"),
        title: String::from("Sales Engineer"),
        phone: String::from("860-249-6908"),
        email: String::from("bBrecher@fischer-technology.com"),
        street_address: String::from("750 Marshall Phelps Rd"),
        state: String::from("CT"),
        zip_code: 06095,
    };

    println!("{}", contact4);
}
