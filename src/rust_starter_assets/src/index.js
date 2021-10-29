import { rust_starter } from "../../declarations/rust_starter";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with rust_starter actor, calling the greet method
  const greeting = await rust_starter.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
