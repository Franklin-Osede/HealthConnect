use onnxruntime::{environment::Environment, session::Session};

fn load_facenet_model() -> Result<Session<'static>, Box<dyn std::error::Error>> {
    let env = Environment::builder().build()?;
    let model_path = "../facenet.onnx"; 
    let session = env.new_session(model_path)?;

    println!("Modelo FaceNet cargado correctamente desde: {}", model_path);
    Ok(session)
}

fn main() {
    match load_facenet_model() {
        Ok(_) => println!("Verificación facial lista para usarse."),
        Err(e) => eprintln!("Error al cargar el modelo: {:?}", e),
    }
}
