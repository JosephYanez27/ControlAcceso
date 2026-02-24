async function cargarMenu() {

    const token = localStorage.getItem("token");

    const response = await fetch("/api/seguridad/menu", {
        headers: {
            "Authorization": "Bearer " + token
        }
    });

    if (!response.ok) {
        window.location.href = "/static/login.html";
        return;
    }

    const permisos = await response.json();

    const menu = document.getElementById("menu");

    if (permisos.seguridad) {
        let seguridad = document.createElement("li");
        seguridad.innerHTML = "Seguridad";

        let sub = document.createElement("ul");

        if (permisos.usuarios)
            sub.innerHTML += `<li><a href="#">Usuarios</a></li>`;

        if (permisos.perfiles)
            sub.innerHTML += `<li><a href="#">Perfiles</a></li>`;

        if (permisos.permisos)
            sub.innerHTML += `<li><a href="#">Permisos</a></li>`;

        seguridad.appendChild(sub);
        menu.appendChild(seguridad);
    }
}