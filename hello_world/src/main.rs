fn main() {
    //Değişken tanımlama

    let mut a = 10;
    let isim_soyisim = "Abdullah Özkoç";
    // println!("İsim Soyisim {} puanı {}",isim_soyisim,a);

    a = 15;

    //Shadowing
    let x = 5;

    let x = {
        let x = x * 2; // buradaki x ilk tanımladığım x
        println!("{}", x); // çarpım sahibi olan x

        x * 2
    };

    println!("{:?}", x); // ikinci çarpım işleminin sonucu x

    let x = "Abdullah";
    println!("{}", x);

    //Constants
    const MAX_DEGER: u32 = 50;

    //Veri Tipleri

    //skaler tipler: integer, floating-point,boolean,char

    //unsigned integer veri tipi tanımlama

    let c: u8 = 20; //  2^8 -1 8 bitlik unsigned bir değişkenin alabileceği maks değer 255'dir. 0-255

    let d: usize = 20;

    //signed integer veri tipi tanımlama

    let c: i8 = -25; // -2^n-1 'den 2^n-1 -1

    //floating point number

    let a = 10.5;

    //boolean

    let tr = true;
    let fl = false;

    //char veri tipi

    let a = 'A';

    //iki bileşik veri itpi vardır. tuple, array

    //tuple

    let a = (10, 5.4, -2, "abdullah");

    let on = a.0;
    let bes_noktadort = a.1;

    //array

    let arr = [1, 2, 3];

    let bir = arr[0];
    let iki = arr[1];

    let arr = [1; 100];

    let meyve = ["elma", "portakal"];
}
