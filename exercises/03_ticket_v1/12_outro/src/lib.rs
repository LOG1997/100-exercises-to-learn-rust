// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`. 
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order{
   pub fn new(product_name:String,quantity:u32,unit_price:u32)->Order{
        Order::valid_product_name(&product_name);
        Order::valid_quantity(&quantity);
        Order::valid_uint_price(&unit_price);

        Order{product_name,quantity,unit_price}
    }
   pub fn total(&self)->u32{
        &self.quantity*&self.unit_price
    }
   pub fn product_name(&self)->&String{
        &self.product_name
    }
   pub fn quantity(&self)->&u32{
        &self.quantity
    }
   pub fn unit_price(&self)->&u32{
        &self.unit_price
    }
   pub fn set_product_name(&mut self,product_name:String){
        Order::valid_product_name(&product_name);
        self.product_name = product_name;
    }

   pub fn set_quantity(&mut self,quantity:u32){
        Order::valid_quantity(&quantity);
        self.quantity = quantity;
    }

   pub fn set_unit_price(&mut self,unit_price:u32){
        Order::valid_uint_price(&unit_price);
        self.unit_price = unit_price;
    }

    fn valid_product_name(product_name:&String)->bool{
        println!("product_name len:{}",product_name.len());
        if product_name.is_empty(){
            panic!("product_name cannot be empty")
        }
        else if product_name.len()>300 {
            panic!("product_name cannot be longer than 300 bytes")
        }
        else {
            return true;
        }
    }
    fn valid_quantity(quantity:&u32)->bool{
        if *quantity == 0{
            panic!("quantity must be strictly greater than zero")
        }
        else {
            return true;
        }
    }
    fn valid_uint_price(unit_price:&u32)->bool{
        if *unit_price == 0{
            panic!("unit_price must be strictly greater than zero")
        }
        else {
            return true;
        }
    }
}

#[cfg(test)]

mod tests {
    use super::Order;
    #[test]
    fn works(){
        let mut order=Order::new("product_1".into(), 21, 12);
        order.set_product_name("product_2".into());
        order.set_quantity(12);
        order.set_unit_price(1);

        assert_eq!(order.product_name(),"product_2");
        assert_eq!(order.quantity(),&12);
        assert_eq!(order.unit_price(),&1);
    }

    #[test]
    #[should_panic(expected = "product_name cannot be empty")]
    fn product_name_cannot_be_empty(){
        Order::new("".into(), 21, 12);
    }

    #[test]
    #[should_panic(expected = "product_name cannot be longer than 300 bytes")]
    fn product_name_cannot_be_longer_than_300_bytes(){
        Order::new("product_name_cannot_be_longer_than_300_bytes_random_intro_github_thanks_;ink_version_setup_cube_go_lang_kong_into_out_stdout_error_bytes_longer_test_seconds_name_result_failed_graph_rust_insert_launch_expect_han_filter_finish_output_port_commit_overlay_title_cents_should_setters_type_other_information_attention_location_meaning_pub_special_access_price_located_will_cant_crate_private_for_cargo_pay_you_more_new".into(), 21, 12);
    }

    #[test]
    #[should_panic(expected = "quantity must be strictly greater than zero")]
    fn quantity_must_be_strictly_greater_than_zero(){
        Order::new("product_1".into(), 0, 12);
    }

    #[test]
    #[should_panic(expected = "unit_price must be strictly greater than zero")]
    fn unit_price_must_be_strictly_greater_than_zero(){
        Order::new("product_1".into(), 21, 0);
    }

    #[test]
    fn total_is_correct(){
        let order=Order::new("product_1".into(), 21, 12);
        assert_eq!(order.total(),21*12);
    }
}