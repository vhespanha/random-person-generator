use csv::WriterBuilder;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Person {
    name: String,
    age: u32,
    profession: String,
    email: String,
}

fn generate_person(used_names: &mut HashSet<String>) -> Person {
    let mut rng = thread_rng();

    //Banco com os nomes
    let first_names = [
        "Luciana",
        "Rafael",
        "Isabela",
        "Marcelo",
        "Cristina",
        "Diego",
        "Camila",
        "Thiago",
        "Larissa",
        "Alexandre",
        "Fernanda",
        "Leonardo",
        "Amanda",
        "Gabriel",
        "Juliana",
        "Rodrigo",
        "Carolina",
        "Lucas",
        "Patricia",
        "André",
    ];
    let first_name = rng.gen_range(0..first_names.len());
    let first_name = first_names[first_name].to_string();

    //Banco com sobrenomes
    let second_names = [
        "Silva",
        "Santos",
        "Oliveira",
        "Pereira",
        "Rodrigues",
        "Ferreira",
        "Almeida",
        "Gomes",
        "Martins",
        "Rocha",
        "Carvalho",
        "Ribeiro",
        "Melo",
        "Costa",
        "Araújo",
        "Cavalcanti",
        "Barbosa",
        "Lima",
        "Nascimento",
        "Fernandes",
    ];
    let second_name = rng.gen_range(0..second_names.len());
    let second_name = second_names[second_name].to_string();

    //Junta nome e sobrenome para formar a variavel nome e e-mail
    let name = format!("{} {}", first_name, second_name);

    //Verifica a existênca de combinações de nome e sobrenome já criados
    if used_names.contains(&name) {
        return generate_person(used_names);
    }

    used_names.insert(name.clone());

    //Geração de idade entre 18 e 65 anos
    let age = rng.gen_range(18..65);

    //Banco de profissões
    let professions = [
        "doctor",
        "worker",
        "police officer",
        "firefighter",
        "developer",
    ];
    let profession = rng.gen_range(0..professions.len());
    let profession = professions[profession].to_string();

    //Geração de e-mail juntando nome a um dominio imaginario
    let email = format!("{}@example.com", name.to_lowercase().replace(" ", "."));

    //Definição final da struct
    Person {
        name,
        age,
        profession,
        email,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:12345")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_client(&mut stream)?;
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stream.read_to_string(&mut input)?;

    let num_rows: u32 = input.trim().parse()?;
    let mut used_names = HashSet::new();
    let mut csv_data = Vec::new();

    {
        let mut writer = WriterBuilder::new().from_writer(&mut csv_data);

        writer.write_record(&["Name", "Age", "Profession", "Email"])?;

        for _ in 0..num_rows {
            let person = generate_person(&mut used_names);
            writer.write_record(&[
                &person.name,
                &person.age.to_string(),
                &person.profession,
                &person.email,
            ])?;
        }

        writer.flush()?;
    }

    stream.write_all(&csv_data)?;

    Ok(())
}
