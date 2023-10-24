// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

pub fn main() {
    let mut  tickets: Vec<Ticket> = Vec::new();

    let standard_ticket = Ticket::Standard(80.00);
    let back_stage_ticket = Ticket::Backstage(200.00,"Vanessa".to_owned());
    let vip_ticket = Ticket::Vip(500.00,"Sarah".to_owned());
    tickets.push(standard_ticket);
    tickets.push(back_stage_ticket);
    tickets.push(vip_ticket);

    for (_, ticket) in tickets.iter().enumerate() {
        match ticket {
            Ticket::Backstage(price, holder) => println!("\nTicket type: Backstage\nholder: {}\nprice: ${}\n", holder, price),
            Ticket::Standard(price) => println!("\nTicket type: Standard\nholder: N/A\nprice: ${}\n", price),
            Ticket::Vip(price, holder) => println!("\nTicket type: VIP\nholder: {}\nprice: ${}\n", holder, price),
            
        }
    }


}


pub enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),

}
    

