async function cargarModulos() {

    const token = localStorage.getItem("token");

    const response = await fetch("/api/modulos", {
        headers: {
            "Authorization": "Bearer " + token
        }
    });

    const modulos = await response.json();
    const menu = document.getElementById("menu");

    menu.innerHTML = "";

    modulos.forEach(m => {
        const li = document.createElement("li");
        li.textContent = m.nombre;
        menu.appendChild(li);
    });
}

function configurarBotones(permisos) {

    const btnCrear = document.getElementById("btnCrear");
    const btnEditar = document.getElementById("btnEditar");
    const btnEliminar = document.getElementById("btnEliminar");

    btnCrear.style.display = permisos.crear ? "inline-block" : "none";
    btnEditar.style.display = permisos.editar ? "inline-block" : "none";
    btnEliminar.style.display = permisos.eliminar ? "inline-block" : "none";
}