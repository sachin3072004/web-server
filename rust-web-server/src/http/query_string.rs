use std::collections::HashMap;
#[derive(Debug)]
pub enum QueryValues<'a>{
    Single(&'a str),
    MultiValue(Vec<&'a str>)
}

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, QueryValues<'a>>
}

impl<'a> From <&'a str> for QueryString<'a>{
    fn from(s: &'a str) -> Self {
        let result:Vec<_> = s.split("&").collect();
        let mut data:HashMap<&'a str, QueryValues<'a>> = HashMap::new();
        for v in result {
            if let Some(index) = v.find("="){
                let k = &v[..index];
                let v = &v[index+1..];
                if let Some(containedValue) = data.get_mut(k) {
                    match containedValue{
                        QueryValues::Single(s) => {
                            let v1 = vec![s,v];
                            data.insert(k, QueryValues::MultiValue(v1));
                        },
                        QueryValues::MultiValue(ref mut s)=>{
                            s.push(v);
                        }
                    }

                }else{
                    data.insert(k,QueryValues::Single(v));
                }
            }else{
            }
        }
        return QueryString{data:data};
    }
}
