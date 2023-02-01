//create a function that crawls scores from NFL.com
fn main() {
    //create a new client
    let client = Client::new();
    //create a new request
    let mut res = client.get("http://www.nfl.com/scores/2016/REG1").send().unwrap();
    //check if the request was successful
    assert!(res.status == hyper::Ok);
    //read the response body
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    //print the response body
    println!("{}", body);
}
