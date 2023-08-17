extern crate q1tsim; // q1tsim kütüphanesi dışarıdan kullanılıyor
use q1tsim::{circuit, gates};
use std::io;

fn main() {
    let mut electron_num = 0;
    let mut entangled_electron = 0;
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Kullanmak istediğiniz elektron sayısını giriniz:");
    io::stdin().read_line(&mut input1).expect("Giriş okunamadı.");
    println!("Kaç adet dolanık elektron yapmak istiyorsunuz:");
    io::stdin().read_line(&mut input2).expect("Giriş okunamadı.");

    electron_num = input1.trim().parse().expect("Geçersiz sayı");
    entangled_electron = input2.trim().parse().expect("Geçersiz sayı");

    let mut circuit = circuit::Circuit::new(entangled_electron, entangled_electron);

    for number in 0..entangled_electron {
        //println!("circuit.h({});", number);
        circuit.h(number);
    }

    circuit.measure_all(&[0, 1, 2]);
    circuit.execute(electron_num);

    let hist = circuit.histogram_string().unwrap();
    let mut sorted_hist = hist.iter().collect::<Vec<_>>();
    sorted_hist.sort_by_key(|(_, count)| *count);

    for (bits, count) in sorted_hist {
        println!("{}: {}", bits, count);
    }
}
