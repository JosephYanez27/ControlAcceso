async function login() {

    const nombre = document.getElementById("nombre").value;
    const pwd = document.getElementById("pwd").value;

    const response = await fetch("/login", {   // 👈 tu endpoint es /login
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ nombre, pwd })
    });

    if (response.ok) {

        const token = await response.json();  // 👈 aquí recibes el string directamente

        localStorage.setItem("token", token);

        window.location.href = "/static/index.html";

    } else {
        alert("Credenciales incorrectas");
    }
}