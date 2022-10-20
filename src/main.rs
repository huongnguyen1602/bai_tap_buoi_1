fn bai_1(){
    let org_arr=[1,2,3,5,8,10,11,3,5,7];
    let sub_arr =[3,5,6];
    let mut bool = false;
    if sub_arr.len() <= org_arr.len(){
       for i in 0..org_arr.len(){
          if sub_arr[0] == org_arr[i]{
             if org_arr.len()-i>= sub_arr.len(){
                /*let mut dem =1;
                for j in 1..sub_arr.len(){
                   if sub_arr[j] == org_arr[i+j]{
                      dem = dem +1;
                   }else{
                      break;
                   }
                }
                if dem == sub_arr.len() {
                   bool = true;
                   break;
                }*/
                let new = &org_arr[i..i+sub_arr.len()];
                if new == sub_arr {
                   bool = true;
                   break;
                }
             }
          }
       }
    }
    println!("Dãy có là dãy con hay không?  {}",match bool {
        true => "có",
        false => "không phải",
    });
 }
 ///////////////////////////////////////
 fn bai_2() {
    println!("Input string: "); 
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.truncate(1);
    let mut org_string = String::from("abcdaeda");
    /*let mut output = String::new();
    let mut dem = 0;
    for i in org_string.chars(){
        if i.to_string() == input {
            dem = dem + 1;
        }else{
            output.push(i);
        }
    }
    println!("Số lần xuất hiện của {} là {}",input, dem);
    println!("Chuỗi không chứa {} là {}",input, output);*/
    println!("Số lần xuất hiện của {} là {}",input,org_string.matches(&input).count());
    let output :String = org_string.replace(&input, "");
    println!("Chuỗi không chứa {} là {}",input, output);
}
fn main() {
    bai_1();
    //bai_2();
}
