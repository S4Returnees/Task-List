<?php
// Récupération des données
$username = $_POST['username'];
$password = $_POST['password'];

// exemple de connexion
$correct_user = "admin";
$correct_pass = "1234";

if ($username === $correct_user && $password === $correct_pass) {
    header("Location: index.html");
    exit();
} else {
    echo "Nom d'utilisateur ou mot de passe incorrect.";
}
?>