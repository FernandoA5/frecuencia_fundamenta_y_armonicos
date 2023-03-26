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

    for i in 1..(cantidad_de_armonicos_posibles+1) as usize{
        
    }


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

    //Ahora repetimos el proceso pero tomando el siguiente valor minimo sin conciderar los que ya fueronalmacenados en id_valores_minimos
    while (contador as f64) < (registros.len() as f64) * 0.1{
        minimo=obtener_valor_minimo(&registros, &id_valores_minimos);
        //println!("Valor minimo: {}", minimo);
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
        // println!("Veces que se repite el valor minimo: {}", contador);
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
