use rug::Integer as big;

#[derive(Debug)]
pub struct E521 {
    pub x: big,  //x-coord
    pub y: big,  //y coord
    pub p: big,  //prime defining finite field
    pub d: big,  //d param for curve
    pub r: big,  //order of curve
    pub n: big,  //number of points
}

impl Clone for E521 {
    fn clone(&self) -> E521 {
        E521 {
            x: self.x.clone(),
            y: self.y.clone(),
            p: self.p.clone(),
            d: self.d.clone(),
            r: self.r.clone(),
            n: self.n.clone(),
        }
    }
}
#[derive(Debug)]
pub struct SymmetricCryptogram {
    pub z: Vec<u8>,     //nonce
    pub c: Vec<u8>,     //ciphertext
    pub t: Vec<u8>      //authentication tag
}
#[derive(Debug)]
pub struct ECCryptogram {
    pub z_x: big,    // Z_x is the x coordinate of the public nonce
    pub z_y: big,    // Z_y is the y coordinate of the public nonce
    pub c: Vec<u8>,     // c represents the ciphertext of an encryption
    pub t: Vec<u8>      // t is the authentication tag for the message
}
#[derive(Debug)]
pub struct KeyObj {

    // id: String,             //Represents the unique ID of the key
    pub owner: String,          //Represents the owner of the key, can be arbitrary
    // key_type: String,        /*Acceptable values are PUBLIC or PRIVATE.
	                        //  PUBLIC keys are used only for encryptions, while keys labeled PRIVATE
	                        //  encrypt or decrypt.*/
    pub pub_key_x: String,       //big.Int value representing E521 X coordinate
    pub pub_key_y: String,       //big.Int value representing E521 X coordinate
    pub priv_key: Vec<u8>,        //big.Int value representing secret scalar, nil if KeyType is PUBLIC
    pub date_created: String,    //Date key was generated
    // signature: String,       //Nil unless PUBLIC. Signs 128 bit SHA3 hash of this KeyObj

}


///module for curve and e521 functionality
pub mod curve{
    pub mod e521;
}

// module for sha3 primitives
pub mod sha3{
    pub mod keccakf;
    pub mod sponge;
    pub mod aux_functions;
}

//module for model functions
pub mod model;

//module for gui-related functions
pub mod view{
    pub mod window;
}
