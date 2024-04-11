use vitium_client::{addtxt, setquit,init};
fn main(){
    let mut siv = cursive::default();
    let ss = &mut siv;
    init(ss,"style.toml".to_string());
    addtxt(ss,"Hello,world. Press q to exit.".to_string());
    setquit(ss,'q');
    ss.run();
    println!("Enter your message here:");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    addtxt(ss,line);
    ss.run();
}
