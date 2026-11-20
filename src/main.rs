fn main() {
    use std::io;
    let mut nbr_mystere:u8=0;
    let mut nombre:u8;
    let mut count:u8= 0;
    use std::process::Command;
    
    println!("Bienvenue dans le nombre mystère, votre but est de trouvé le nombre entre 1 et 100 bonne chance\n\n");
    loop {
        if count==10
        {
            println!("Tu as perdu, tu n'as pas plus de 10 essaies");
            break;
        }
        loop {
            print!("Écris le nombre que tu veux:\n");
            let mut saisie = String::new();
            io::stdin().read_line(&mut saisie).expect("Erreur de lecture");
            nombre=match saisie.trim().parse() {
                Ok(num) => {
                    if num>100 || num<1 {
                        println!("Choisissez un nombre valable");
                        continue;
                    }
                    else {num}
                }
                Err(_) => {
                    println!("Veuillez entrer un bon nombre");
                    continue;
                }
            };
            if count==0 {nbr_mystere = rand::random_range(1..=100);}
            break;
        }
        Command::new("clear").status().unwrap();
        if nombre==nbr_mystere{
            count+=1;
            println!("Bien joué tu as trouvé le nombre mystère\n");
            println!("Vous avez fait {} essaie\n",count);
            break;
        }
        if nombre>nbr_mystere{
            count+=1;
            println!("Ton nombre est trop petit ({} essaie restant)\n",10-count);
            continue;
        }
        if nombre<nbr_mystere{
            count+=1;
            println!("Ton nombre est trop grand ({} essaie restant)\n",10-count);
            continue;
        }
    }

}

