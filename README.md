# Sesame

Ce projet propose un POC d'application permettant à divers utilisateurs d'agir autour de toilettes publiques connectées.

# Lancement

- Se rendre dans le dossier `docker`
- Lancer la command `docker-compose up`

Le premier lancement est plutôt long puisqu'il télécharge les ressources nécessaires pour les images.

Il y a en tout 3 containers pour l'application :
- La base de donnée
- L'application Angular (frontend)
- L'application Rust (backend)

Il y a aussi 3 containers (ELK) pour le monitoring. Pour lancer l'application sans ELK, utilisez la commande `docker-compose up backend frontend`.

## Développement

Notamment pour le frontend, il faut lancer le container, puis faire un `npm i` dans le dossier frontend. Ce qui permet d'avoir les packages sur la machine pour développer.

Pour la backend, si vous êtes sous windows, il faut redémarrer le container à chaque modification car le cargo watch ne fonctionne pas.