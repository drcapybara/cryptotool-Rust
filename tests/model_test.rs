#[cfg(test)]
pub mod model_test {
    use std::{time::Instant, borrow::BorrowMut};
    use cryptotool::{
        model::shake_functions::{encrypt_with_pw, decrypt_with_pw, gen_keypair, encrypt_with_key, decrypt_with_key, sign_with_key, verify_signature}, 
        curve::e521::e521_module::get_e521_point};
    use cryptotool::sha3::aux_functions::byte_utils::get_random_bytes;

    #[test]
    pub fn test_sym_enc() {
        let mut total = 0.0;
        let rounds = 10.0;
        for _ in 0..rounds as i32 {
            let pw = get_random_bytes(16);
            let mut message = Box::new(get_random_bytes(5242880).to_owned());
            // let mut message = hex::decode("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7").unwrap();
            let now = Instant::now();
            let mut cg2 = Box::new(encrypt_with_pw(&mut pw.clone(), &mut message));
            let res = decrypt_with_pw(&mut pw.clone(), &mut cg2.borrow_mut());
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            total += sec;
            assert!(res);
        }
        println!("Code took: {} seconds", total / rounds);
}

    #[test]
    fn test_key_gen_enc_dec() { //check conversion to and from bytes.

        let mut total = 0.0;
        let rounds = 10.0;
        for _ in 0..rounds as i32 {
            let pw = get_random_bytes(16);
            let owner = "test key".to_string();
            let mut message = Box::new(get_random_bytes(5242880).to_owned()); //5mb
            // let mut message = Box::new(hex::decode("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7").unwrap());    
            let key_obj = gen_keypair(&mut pw.clone(), owner);
            let x = key_obj.pub_x;
            let y = key_obj.pub_y;
            let mut pub_key = get_e521_point(x, y);
            let now = Instant::now();
            let mut enc = encrypt_with_key(&mut pub_key, &mut message);
            // println!("enc{:?}: ", hex::encode(enc.c.clone()));
            let res = decrypt_with_key(&mut pw.clone(), enc.borrow_mut());
            // println!("dec{:?}: ", hex::encode(enc.c));
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            total += sec;
            assert!(res);
        }
        println!("Code took: {} seconds", total / rounds);
    }

    #[test]
    pub fn test_signature() {
        // let mut message = get_random_bytes(5242880);
        let mut message = Box::new(get_random_bytes(5242880).to_owned());        
        let pw = get_random_bytes(16);
        let key_obj = gen_keypair(&mut pw.clone(), "test".to_string());
        let x = key_obj.pub_x;
        let y = key_obj.pub_y;
        let mut key = get_e521_point(x, y);
        let sig = sign_with_key(&mut pw.clone(), &mut message);
        let res = verify_signature(&sig, &mut key, &mut message);
        println!("good sig?: {:?}", res);
    }

}