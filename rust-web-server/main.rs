#[derive(Debug)]
struct Profile{
    name: &Name,
    address: &Address
}

#[derive(Debug)]
struct Name{
    firstName: &str,
    lastName: &str
}

#[derive(Debug)]
struct Address{
    num: i32,
    firstLine: &str,
    city: &str,
    state: &str,
}

fn testing(firstName:&str,lastName:&str,firstline:&str,city:&str,state:&str)->Profile{
    return Profile{name:&Name{firstName:firstName,lastName:lastName},address:&Address{num:10,firstLine:firstLine,city:city,state:state}};
}
fn main() {
    let firstName = String ::from("Sachin");
    {
        let lastName = String::from("Gupta");
        let n = Name{firstName:&firstName, lastName:&lastName};
        {
            let firstline = String::from("n.sienna st");
            {
                let city = String::from("Visalia");
                {
                    let state = String::from("CA");
                    let p = Profile{name:&n,address:&Address{num:10, firstLine:&firstline,city:&city,state:&state} };
                    println!("{:?}",p);
                    let p1 = testing(&firstName,&lastName,&firstline,&city,&state);
                    println!("{:?}",p1);
                }

            }
        }


    }
}
