async function cargarPermisos() {
    try {
        const perfilId = document.getElementById("filtroPerfil").value;
        if (!perfilId) return;

        const token = localStorage.getItem("token");

        const response = await fetch(`/api/seguridad/permisos/${perfilId}`, {
            headers: {
                "Authorization": "Bearer " + token
            }
        });

        if (response.status === 401) {
            alert("Sesión expirada");
            window.location.href = "/login.html";
            return;
        }

        if (!response.ok) {
            throw new Error("Error al cargar permisos");
        }

        const data = await response.json();
        const tabla = document.getElementById("tablaPermisos");
        tabla.innerHTML = "";

        data.forEach(p => {
            const fila = document.createElement("tr");

            fila.dataset.idModulo = p.id_modulo;
            fila.dataset.idPermiso = p.id ?? "";

            fila.innerHTML = `
                <td>${p.nombre ?? "Módulo"}</td>
                <td><input type="checkbox" class="chkCrear" ${p.crear ? "checked" : ""}></td>
                <td><input type="checkbox" class="chkEditar" ${p.editar ? "checked" : ""}></td>
                <td><input type="checkbox" class="chkEliminar" ${p.eliminar ? "checked" : ""}></td>
            `;

            tabla.appendChild(fila);
        });

    } catch (error) {
        console.error(error);
        alert("No se pudieron cargar los permisos");
    }
}
async function guardarPermisos() {
    const btn = document.getElementById("btnGuardar");

    try {
        btn.disabled = true; // 🔒 deshabilita
        btn.textContent = "Guardando...";

        const perfilId = document.getElementById("filtroPerfil").value;
        if (!perfilId) {
            alert("Selecciona un perfil");
            return;
        }

        const token = localStorage.getItem("token");
        const filas = document.querySelectorAll("#tablaPermisos tr");

        const permisos = [];

        filas.forEach(fila => {
            permisos.push({
                id: fila.dataset.idPermiso || null,
                id_modulo: parseInt(fila.dataset.idModulo),
                id_perfil: parseInt(perfilId),
                crear: fila.querySelector(".chkCrear").checked,
                editar: fila.querySelector(".chkEditar").checked,
                eliminar: fila.querySelector(".chkEliminar").checked
            });
        });

        const response = await fetch("/api/seguridad/permisos", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token
            },
            body: JSON.stringify(permisos)
        });

        if (response.status === 401) {
            alert("Sesión expirada");
            window.location.href = "/login.html";
            return;
        }

        if (!response.ok) {
            throw new Error("Error al guardar permisos");
        }

        alert("Permisos guardados correctamente");

    } catch (error) {
        console.error(error);
        alert("No se pudieron guardar los permisos");
    } finally {
        btn.disabled = false; // 🔓 habilita otra vez
        btn.textContent = "Guardar";
    }
}