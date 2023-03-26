#[derive(Debug, Clone)]
struct Registro{
    registro: i32,
    time: f64,
    value: f64
}
fn main() {
    //Leer Archivo CSV
    let mut rdr = csv::Reader::from_path("guitarra.autoc.csv").unwrap();
    //arraylist de registros
    let mut registros: Vec<Registro> = Vec::new();

    //VER CONTENDIO 
    for result in rdr.records() {
        let record = result.unwrap();
        let registro = Registro{
            registro: obtener_numero_i32(record[0].parse::<i32>().ok()).unwrap(),
            time: obtener_numero_f64(record[1].parse::<f64>().ok()).unwrap(),
            value: obtener_numero_f64(record[2].parse::<f64>().ok()).unwrap(),
        };
        registros.push(registro.clone());
        //println!("{:?}", registro);
    }

    //VER FRECUENCIA FUNDAMENTAL
    let frecuencia_fundamental = ver_frecuencia_fundamental(&registros);
    println!("Frecuencia fundamental: {}", frecuencia_fundamental);
    let frecuencia_maxima = frecuencia_maxima(&registros);
    println!("Frecuencia máxima: {}", frecuencia_maxima);

    let cantidad_de_armonicos_posibles = (frecuencia_maxima / frecuencia_fundamental) as i32;
    println!("Cantidad de armónicos: {}", cantidad_de_armonicos_posibles);

    let mut armonicos_repetidos: Vec<Registro> = Vec::new();
    for i in 1..(cantidad_de_armonicos_posibles+1) as usize{
        let registro: Option<Registro> = evaluar_armonico(&registros, frecuencia_fundamental, i as i32, &armonicos_repetidos);
        if registro.is_some(){
            armonicos_repetidos.push(registro.unwrap());
        }
    }
}
fn evaluar_armonico(registros: &Vec<Registro>, frecuencia_fundamental: f64, num_armoninco: i32, armonicos_repetidos: &Vec<Registro>)-> Option<Registro>{
    let armonico = frecuencia_fundamental * (num_armoninco as f64);
    let mut armonico_encontrado: Option<Registro> = None;
    let mut contador_apariciones_armonico= 0;
    for registro in registros.iter(){
        let mut saltar = false;
        if armonicos_repetidos.len() != 0{
            for armonico_repetido in armonicos_repetidos.iter(){
                if registro.registro == armonico_repetido.registro{
                    saltar = true;
                }
            }
        }
        if registro.value < armonico*1.02 && registro.value > armonico*0.98 && saltar == false{
            contador_apariciones_armonico += 1;
            if contador_apariciones_armonico == 1{
                armonico_encontrado = Some(registro.clone());
                print!("Armonico: {} | Id: {} | V_Cal: {} | V_Enc: {} | T: {} | ", num_armoninco, registro.registro, armonico, registro.value, registro.time);
            }
        }
    }
    if contador_apariciones_armonico != 0 {
        println!("Apariciones: {}", contador_apariciones_armonico);
    }
    armonico_encontrado
}
fn frecuencia_maxima(registros: &Vec<Registro>)-> f64{
    let mut maximo = registros[0].value;
    for registro in registros.iter(){
        if registro.value > maximo{
            maximo = registro.value;
        }
    }
    maximo
}
fn ver_frecuencia_fundamental(registros: &Vec<Registro>)-> f64{
    //Lista de ids de los valores minimos ya evaluados
    let mut id_valores_minimos: Vec<Registro> = Vec::new();
    let mut minimo = registros[0].value;
    let mut contador_maximo =0;
    let mut contador =0;

    while (contador as f64) < (registros.len() as f64) * 0.1{
        minimo=obtener_valor_minimo(&registros, &id_valores_minimos);
        //Vemos cuantas veces se repitió el valor mínimo con una tolerancia de +- 10%
        contador = 0;
        for registro in registros.iter(){
            for ya_evaluado in id_valores_minimos.iter(){
                if registro.registro == ya_evaluado.registro{
                    continue;
                }
            }
            if registro.value < minimo*1.1 && registro.value > minimo*0.9{
                contador += 1;
                id_valores_minimos.push(registro.clone());
            }
        }
        if contador > contador_maximo{
            contador_maximo = contador;
        }
    }
    println!("Apariciones de la frecuencia fundamental: {}", contador_maximo);
    minimo
}

fn obtener_valor_minimo(registros: &Vec<Registro>, ya_evaluados: &Vec<Registro>)-> f64{
    //Obtenemos el valor minimo de registro.value
    let mut minimo: f64;
    if ya_evaluados.len() == 0{
        minimo = registros[0].value;
    }else{
        minimo = frecuencia_maxima(registros);
        //println!("Cambio de minimo: {}", minimo);      
    }
    let mut next= false;
    for registro in registros.iter(){
        for ya_evaluado in ya_evaluados.iter(){
            if registro.registro == ya_evaluado.registro{
                next = true;
            }
        }
        if next == true {continue;}
        if registro.value < minimo{
            minimo = registro.value;
        }
    }
    minimo
}
fn obtener_numero_f64(numero: Option<f64>) -> Option<f64>{
    match numero {
        Some(numero) => Some(numero),
        None => None,
    }
}
fn obtener_numero_i32(numero: Option<i32>) -> Option<i32>{
    match numero {
        Some(numero) => Some(numero),
        None => None,
    }
}
