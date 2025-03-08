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
* **Ruoli autorizzati**: Council, Officer

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
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord name 123456789 "Nuovo Nome Server"`

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
* **Ruoli autorizzati**: Balance Manager, Council, Officer
* **Esempio comando Discord**: `!discord balance 123456789 1000`

### `[GET] /discords/{discord_id}/balance`

* **Descrizione**: Recupera il bilancio associato a un server Discord.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Dettagli**: Restituisce il valore del bilancio del server.
* **Ruoli autorizzati**: Member, Guest, Council, Officer, Balance Manager
* **Esempio comando Discord**: `!discord getbalance 123456789`

### `[PATCH] /discords/{discord_id}/left`

* **Descrizione**: Imposta se il bot ha lasciato il server.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "is_bot_left": true/false
    }
    ```

* **Dettagli**: Imposta se il bot ha lasciato il server.
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord left 123456789 true`

### `[GET] /discords/{discord_id}/left`

* **Descrizione**: Recupera se il bot ha lasciato il server.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Dettagli**: Recupera se il bot ha lasciato il server.
* **Ruoli autorizzati**: Member, Guest, Council, Officer
* **Esempio comando Discord**: `!discord getleft 123456789`

### `[PATCH] /discords/{discord_id}/application`

* **Descrizione**: Abilita/Disabilita la funzione application.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "true/false": true/false
    }
    ```

* **Dettagli**: Abilita/Disabilita la funzione application.
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord application 123456789 true`

### `[PATCH] /discords/{discord_id}/content`

* **Descrizione**: Abilita/Disabilita la funzione content.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "true/false": true/false
    }
    ```

* **Dettagli**: Abilita/Disabilita la funzione content.
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord content 123456789 true`

### `[PATCH] /discords/{discord_id}/logs`

* **Descrizione**: Abilita/Disabilita la funzione logs.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "true/false": true/false
    }
    ```

* **Dettagli**: Abilita/Disabilita la funzione logs.
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord logs 123456789 true`

### `[PATCH] /discords/{discord_id}/builds`

* **Descrizione**: Abilita/Disabilita la funzione builds.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "true/false": true/false
    }
    ```

* **Dettagli**: Abilita/Disabilita la funzione builds.
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord builds 123456789 true`

### `[PATCH] /discords/{discord_id}/balance`

* **Descrizione**: Abilita/Disabilita la funzione balance.
* **Parametro URL**: `discord_id` (identificatore del server Discord)
* **Corpo della richiesta (JSON)**:

    ```json
    {
    "true/false": true/false
    }
    ```

* **Dettagli**: Abilita/Disabilita la funzione balance.
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!discord balance 123456789 true`

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
* **Ruoli autorizzati**: Council, Officer

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
* **Ruoli autorizzati**: Council, Officer
* **Esempio comando Discord**: `!user username 123456789 "NuovoNomeUtente"`

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
* **Ruoli autorizzati**: Vod Review, Council, Officer, Points Manager
* **Esempio comando Discord**: `!user vod 123
