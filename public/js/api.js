async function apiFetch(url, options = {}) {

    const token = localStorage.getItem("token");

    options.headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer " + token,
        ...options.headers
    };

    const response = await fetch(url, options);

    if (response.status === 401) {
        alert("Sesión expirada");
        window.location.href = "/login.html";
        return;
    }

    return response.json();
}