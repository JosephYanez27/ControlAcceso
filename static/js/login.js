async function login() {

    const nombre = document.getElementById("nombre").value;
    const pwd = document.getElementById("pwd").value;

    try {
        const response = await fetch("https://controlacceso-l9rs.onrender.com/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ nombre, pwd })
        });

        if (response.ok) {

            const token = await response.json();

            localStorage.setItem("token", token);

            window.location.href = "/index.html"; 
            // 👆 IMPORTANTE
            // Como usas Files::new("/", "./static")
            // NO necesitas /static/

        } else {
            const mensaje = await response.text();
            alert("Error: " + mensaje);
        }

    } catch (error) {
        alert("Error de conexión con el servidor");
        console.error(error);
    }
}