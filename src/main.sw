contract;
 
storage {
    greeting: str[10] = __to_str_array("Hello!!!!!"),
}
 
abi Hello {
    #[storage(read, write)]
    fn change_greeting();
 
    #[storage(read)]
    fn greet() -> str[10];
}
 
impl Hello for Contract {
    #[storage(read)]
    fn greet() -> str[10] {
        storage.greeting.read()
    }
 
    #[storage(read, write)]
    fn change_greeting() {
        let old_greet = from_str_array(storage.greeting.read());
        if (old_greet.eq("Hello!!!!!")){
            storage.greeting.write(__to_str_array("Bonjour!!!"));
        }
        if (old_greet.eq("Bonjour!!!")){
            storage.greeting.write(__to_str_array("Konnichiwa"));
        }
        if (old_greet.eq("Konnichiwa")){
            storage.greeting.write(__to_str_array("Namaste!!!"));
        }
        if (old_greet.eq("Namaste!!!")){
            storage.greeting.write(__to_str_array("Hola!!!!!!"));
        }
        if (old_greet.eq("Hola!!!!!!")){
            storage.greeting.write(__to_str_array("Hello!!!!!"));
        }

        
    }
}