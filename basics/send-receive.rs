//Lets make send & receive 


use std::io;
use rand::Rng;

fn send_bitcoin() {

    println!("\nWe are going to send some bitcoin\n");

    let clients = vec!["Piro","Luci","Draco"];
    println!("Who do you want to send Bitcoin?\n");

    //lets print out clients
    for client in &clients{
        println!("{}", client);
    }
    println!("\n");

    //lets users input for which clients
    let mut receipient = String::new();
    io::stdin().read_line(&mut receipient);    

    if clients.contains(&receipient.trim()) {
        println!("How many bitcoin do you want to send?\n");

        let mut amount = String::new();
        io::stdin().read_line(&mut amount);

        println!("\nYou have send {} bitcoin to {}",amount,receipient);

        println!("{} : Thanks for the Bitcoin",receipient);

    }

    else {
        println!("{} its not in your contact list",receipient.trim());
    }
}



fn receive_bitcoin() {
    println!("\nWe are going to receive some bitcoin\n");

    let amount = rand::thread_rng().gen_range(1..10);
    println!("\n You just received {} bitcoin\n", amount);    
}



fn exit_console () {
    println!("Invalid option, must be (s) or (r)");
}



fn console() {

    println!("\nWELCOME TO DRACO BANK\n");

    println!("\nDo you want to send (s) or receive (r)\n");

    //users input
    let mut command = String::new();
    io::stdin().read_line(&mut command);

    //check users input
    if command.trim().eq("s"){
        send_bitcoin()
    } 
    else if command.trim().eq("r"){
        receive_bitcoin()
    }
    else {
        exit_console()
    }


}

fn main() {
    console()
}
