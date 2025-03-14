fn main() {
  let owner = String::from("Здравствуйте!");  // greeting - владелец строки "Здравствуйте!"
  println!("greeting: {}", owner);

  let new_owner = owner;            // Владение строкой перемещается к another_greeting

  println!("greeting: {}", owner);       // Ошибка! greeting больше не владеет строкой
  println!("another_greeting: {}", new_owner);
}