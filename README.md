### Database:
- Surreal DB

### Todo List:
- discord_controller rimuovere .clone e usare i & riferimenti
- rimuovere .unwrap o .expect s sostituirli con Enum::Result

# API Documentation

## Gestione Discord

### `[PUT] /discords`

* **Descrizione**: Crea o aggiorna un record di Discord.
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "discord_id": "identificatore_discord",
    "discord_name": "nome_server",
    "joined_at": "data_ingresso",
    "...": "altri_dati"
    }
    ```

* **Dettagli**: Utilizzato per registrare o aggiornare le informazioni di un server Discord nel sistema.

### `[PATCH] /discords/{discord_id}/name`

* **Descrizione**: Aggiorna il nome di un server Discord.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "new_name": "nuovo_nome"
    }
    ```

* **Dettagli**: Permette la modifica del nome di un server Discord.

### `[PATCH] /discords/{discord_id}/balance`

* **Descrizione**: Aggiorna il bilancio associato a un server Discord.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "amount": "importo"
    }
    ```

* **Dettagli**: Permette di modificare il valore del bilancio del server.

### `[GET] /discords/{discord_id}/balance`

* **Descrizione**: Recupera il bilancio associato a un server Discord.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Dettagli**: Restituisce il valore del bilancio del server.

## Gestione Utenti

### `[PUT] /users`

* **Descrizione**: Crea o aggiorna un record utente.
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "user_id": "identificatore_utente",
    "username": "nome_utente",
    "balance": "saldo",
    "...": "altri_dati"
    }
    ```

* **Dettagli**: Utilizzato per registrare o aggiornare le informazioni di un utente nel sistema.

### `[PATCH] /users/{user_id}/username`

* **Descrizione**: Aggiorna il nome utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "new_username": "nuovo_nome_utente"
    }
    ```

* **Dettagli**: Modifica il nome utente.

### `[PATCH] /users/{user_id}/vod`

* **Descrizione**: Aggiunge o sottrae punti VOD all'utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "points": "punti"
    }
    ```

* **Dettagli**: Modifica il punteggio VOD dell'utente.

### `[PATCH] /users/{user_id}/link`

* **Descrizione**: Aggiunge o modifica il nome IGN collegato all'utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "ign_name": "nome_ign"
    }
    ```

* **Dettagli**: Collega un nome IGN all'account dell'utente.

### `[PATCH] /users/{user_id}/attendance`

* **Descrizione**: Aggiunge o sottrae punti di presenza all'utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "points": "punti"
    }
    ```

* **Dettagli**: Modifica il punteggio di presenza dell'utente.

### `[PATCH] /users/{user_id}/balance`

* **Descrizione**: Aggiorna il bilancio dell'utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "amount": "importo"
    }
    ```

* **Dettagli**: Modifica il bilancio dell'utente.

### `[DELETE] /users/{user_id}/link`

* **Descrizione**: Rimuove il link IGN dall'utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Dettagli**: Scollega il nome IGN dall'account dell'utente.

### `[GET] /users/{user_id}/balance`

* **Descrizione**: Recupera il bilancio dell'utente.
* **Parametro URL**: `user_id` (identificatore dell'utente)
* **Dettagli**: Restituisce il bilancio dell'utente.

## Gestione Competizioni (Comps)

### `[PUT] /comps`

* **Descrizione**: Crea una nuova competizione.
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "name": "nome_competizione",
    "content": "contenuto",
    "description": "descrizione"
    }
    ```

* **Dettagli**: Utilizzato per registrare una nuova competizione nel sistema.

### `[PATCH] /comps/{comp_id}/name`

* **Descrizione**: Aggiorna il nome di una competizione.
* **Parametro URL**: `comp_id` (identificatore della competizione)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "new_name": "nuovo_nome"
    }
    ```

* **Dettagli**: Modifica il nome della competizione.

### `[PATCH] /comps/{comp_id}/description`

* **Descrizione**: Aggiorna la descrizione di una competizione.
* **Parametro URL**: `comp_id` (identificatore della competizione)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "new_description": "nuova_descrizione"
    }
    ```

* **Dettagli**: Modifica la descrizione della competizione.

### `[DELETE] /comps/{comp_id}`

* **Descrizione**: Elimina una competizione.
* **Parametro URL**: `comp_id` (identificatore della competizione)
* **Dettagli**: Elimina la competizione.

## Gestione Build (Builds)

### `[PUT] /builds`

* **Descrizione**: Crea una nuova build.
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "weapon": "arma",
    "role": "ruolo",
    "cape": "mantello",
    "...": "altri_componenti"
    }
    ```

* **Dettagli**: Permette la creazione di una nuova configurazione di equipaggiamento (build).
