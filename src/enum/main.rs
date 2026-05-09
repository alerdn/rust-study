enum CustomError {
    Generic(&'static str),
    Network(u16, &'static str),
}

impl CustomError {
    fn throw(&self) {
        match self {
            CustomError::Generic(message) => println!("Generic error: {}", message),
            CustomError::Network(status, message) => {
                println!("Network error ({}): {}", status, message)
            }
        }
    }
}

fn main() {
    let generic_error = CustomError::Generic("Client not found");
    let net_error = CustomError::Network(401, "Unauthorized");

    generic_error.throw();
    net_error.throw();
}
