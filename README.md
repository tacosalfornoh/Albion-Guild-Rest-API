### Database:
- Surreal DB

### Todo List:
- discord_controller rimuovere .clone e usare i & riferimenti
- rimuovere .unwrap o .expect s sostituirli con Enum::Result

## Albion Guild REST API Documentation

This document provides a comprehensive guide to the Albion Guild REST API, outlining endpoints, request methods, and data structures for developers looking to integrate with the API.

### Base URL

`[Insert Base URL Here]`  *(Please replace with the actual base URL of your API)*

### Authentication

*   API keys or OAuth 2.0 *(Details on authentication methods should be added here if applicable)*

### Endpoints

This API is structured around the following resources:

*   **Discords:** Managing Discord server data.
*   **Users:** Managing user data within the guild.
*   **Competitions (Comps):** Managing guild competitions.
*   **Builds:** Managing character builds.
*   **Contents:** Managing guild content and events.
*   **Participants (Partecipants):** Managing participant data for contents/events.
*   **Leaderboards:** Accessing guild leaderboards.

---

## 1. Discord Endpoints (`/discords`)

### 1.1. `PUT /discords` - Create Discord Entry

*   **Description:**  Creates a new entry for a Discord server in the database.
*   **Request Method:** `PUT`
*   **Endpoint:** `/discords`
*   **Request Body (JSON):**

    ```json
    {
      "discord_id": "string",      // Unique Discord server ID (required)
      "discord_name": "string",    // Discord server name (required)
      "joined_at": "string",     // Date when the bot joined the server (e.g., "2024-03-08T19:30:00Z")
      "...": "..."                //  Other Discord-related fields as needed
    }
    ```

*   **Example Request (cURL):**

    ```bash
    curl -X PUT "[Base URL]/discords" \
    -H "Content-Type: application/json" \
    -d '{
      "discord_id": "1234567890",
      "discord_name": "Example Guild Discord",
      "joined_at": "2024-03-08T19:30:00Z"
    }'
    ```

*   **Authorized Roles:** `Council`, `Officer`

---

### 1.2. `PATCH /discords/<discord_id>/name` - Update Discord Name

*   **Description:** Updates the name of an existing Discord server entry.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/discords/<discord_id>/name`
*   **Path Parameter:**
    *   `discord_id`:  ID of the Discord server to update.
*   **Request Body (JSON):**

    ```json
    {
      "new_name": "string"     // New Discord server name (required)
    }
    ```

*   **Example Request (cURL):**

    ```bash
    curl -X PATCH "[Base URL]/discords/1234567890/name" \
    -H "Content-Type: application/json" \
    -d '{
      "new_name": "New Example Guild Name"
    }'
    ```

*   **Authorized Roles:** `Council`, `Officer`

---

### 1.3. `PATCH /discords/<discord_id>/balance` - Update Discord Balance Feature Status

*   **Description:** Enables or disables the balance tracking feature for a Discord server.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/discords/<discord_id>/balance`
*   **Path Parameter:**
    *   `discord_id`: ID of the Discord server.
*   **Request Body (JSON):**

    ```json
    {
      "balance": "boolean"    // `true` to enable, `false` to disable (required)
    }
    ```

*   **Example Request (cURL):**

    ```bash
    curl -X PATCH "[Base URL]/discords/1234567890/balance" \
    -H "Content-Type: application/json" \
    -d '{
      "balance": true
    }'
    ```

*   **Authorized Roles:** `Council`, `Officer`

---

### 1.4. `GET /discords/<discord_id>/balance` - Get Discord Balance Feature Status

*   **Description:** Retrieves the current status (enabled/disabled) of the balance tracking feature for a Discord server.
*   **Request Method:** `GET`
*   **Endpoint:** `/discords/<discord_id>/balance`
*   **Path Parameter:**
    *   `discord_id`: ID of the Discord server.

*   **Example Request (cURL):**

    ```bash
    curl -X GET "[Base URL]/discords/1234567890/balance"
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Balance Manager`

---

**(Continue this Markdown format for all API endpoints, following the structure and details from the original README.md file.  For each endpoint, include:**

*   **Endpoint Title:**  e.g., `2. User Endpoints (/users)` , `2.1. PUT /users - Create User Entry`
*   **Description:** A concise explanation of the endpoint's function.
*   **Request Method:** `PUT`, `PATCH`, `GET`, `DELETE`
*   **Endpoint:** The API endpoint path.
*   **Path Parameters:** (If applicable) List and describe path parameters.
*   **Query Parameters:** (If applicable) List and describe query parameters.
*   **Request Body (JSON):** (If applicable)  JSON structure with field names, data types, and descriptions. Use code blocks for JSON examples.
*   **Example Request (cURL):** (Optional but highly recommended) Example cURL command demonstrating how to use the endpoint. Use code blocks for cURL examples.
*   **Authorized Roles:** List of Discord roles authorized to use this endpoint (e.g., `Council`, `Officer`, `Member`).

**... Continue documenting all endpoints in this Markdown format, mirroring the structure and details from the provided README.md file.**

---

**Example of how to continue documenting User Endpoints (`/users`) following the Markdown format:**

## 2. User Endpoints (`/users`)

### 2.1. `PUT /users` - Create User Entry

*   **Description:** Creates a new user entry in the database.
*   **Request Method:** `PUT`
*   **Endpoint:** `/users`
*   **Request Body (JSON):**

    ```json
    {
      "user_id": "string",     // Unique user ID (required)
      "username": "string",   // Username (required)
      "balance": "integer",    // Initial balance (optional, default to 0 or specified value)
      "...": "..."             // Other user-related fields
    }
    ```

*   **Example Request (cURL):**

    ```bash
    curl -X PUT "[Base URL]/users" \
    -H "Content-Type: application/json" \
    -d '{
      "user_id": "user123",
      "username": "TestUser",
      "balance": 100
    }'
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Balance Manager`, `Points Manager`

---

### 2.2. `PATCH /users/<user_id>/username` - Update Username

*   **Description:** Updates the username of an existing user.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/username`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "new_username": "string"  // New username (required)
    }
    ```

*   **Example Request (cURL):**

    ```bash
    curl -X PATCH "[Base URL]/users/user123/username" \
    -H "Content-Type: application/json" \
    -d '{
      "new_username": "UpdatedUsername"
    }'
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Points Manager`

### 2.3. `PATCH /users/<user_id>/vod` - Update VOD Points

*   **Description:** Updates the VOD points for a user.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/vod`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "points": "integer"    // Points to add or set for VOD review (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /users/user123/vod {"points": 5}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Vod Review`, `Points Manager`

---

### 2.4. `PATCH /users/<user_id>/link` - Link In-Game Name

*   **Description:** Links an in-game name to a user.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/link`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "ign_name": "string"   // In-game name to link (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /users/user123/link {"ign_name": "ExampleIGN"}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Application Manager`, `Points Manager`

---

### 2.5. `PATCH /users/<user_id>/attendance` - Update Attendance Points

*   **Description:** Updates the attendance points for a user.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/attendance`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "points": "integer"    // Points to add or set for attendance (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /users/user123/attendance {"points": 10}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Points Manager`, `Content Manager`

---

### 2.6. `PATCH /users/<user_id>/balance` - Update Balance

*   **Description:** Updates the balance of a user.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/balance`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "amount": "integer"    // Amount to add or set to the user's balance (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /users/user123/balance {"amount": 50}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Balance Manager`

---

### 2.7. `DELETE /users/<user_id>/link` - Remove In-Game Name Link

*   **Description:** Removes the linked in-game name from a user.
*   **Request Method:** `DELETE`
*   **Endpoint:** `/users/<user_id>/link`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.

*   **Example Request (Discord Bot):**

    ```
    !api delete /users/user123/link
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Application Manager`, `Points Manager`

---

### 2.8. `GET /users/<user_id>/balance` - Get Balance

*   **Description:** Retrieves the balance of a user.
*   **Request Method:** `GET`
*   **Endpoint:** `/users/<user_id>/balance`
*   **Path Parameter:**
    *   `user_id`: ID of the user to retrieve balance for.

*   **Example Request (Discord Bot):**

    ```
    !api get /users/user123/balance
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Balance Manager`, `Member`, `Guest`

---

### 2.9. `PATCH /users/<user_id>/left` - Update User Bot Left Status

*   **Description:** Updates the status indicating if the bot has left the user's context (e.g., Discord server).
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/left`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "is_bot_left": "boolean" // `true` if bot left, `false` otherwise (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /users/user123/left {"is_bot_left": true}
    ```

*   **Authorized Roles:** `Council`, `Officer`

---

### 2.10. `GET /users/<user_id>/left` - Get User Bot Left Status

*   **Description:** Retrieves the status indicating if the bot has left the user's context.
*   **Request Method:** `GET`
*   **Endpoint:** `/users/<user_id>/left`
*   **Path Parameter:**
    *   `user_id`: ID of the user to retrieve status for.

*   **Example Request (Discord Bot):**

    ```
    !api get /users/user123/left
    ```

*   **Authorized Roles:** `Council`, `Officer`

---

### 2.11. `PATCH /users/<user_id>/donation` - Update Donation Points

*   **Description:** Updates the donation points for a user.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/users/<user_id>/donation`
*   **Path Parameter:**
    *   `user_id`: ID of the user to update.
*   **Request Body (JSON):**

    ```json
    {
      "points": "integer"    // Points to add or set for donation tracking (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /users/user123/donation {"points": 20}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Points Manager`

---

## 3. Competition Endpoints (`/comps`)

### 3.1. `PUT /comps` - Create Competition

*   **Description:** Creates a new competition entry.
*   **Request Method:** `PUT`
*   **Endpoint:** `/comps`
*   **Request Body (JSON):**

    ```json
    {
      "name": "string",        // Name of the competition (required)
      "content": "string",     // Content type of the competition (required)
      "description": "string" // Description of the competition (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api put /comps {"name": "Build Competition", "content": "Open World PvP", "description": "Best open world build competition"}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Content Manager`

---

### 3.2. `PATCH /comps/<comp_id>/name` - Update Competition Name

*   **Description:** Updates the name of an existing competition.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/comps/<comp_id>/name`
*   **Path Parameter:**
    *   `comp_id`: ID of the competition to update.
*   **Request Body (JSON):**

    ```json
    {
      "new_name": "string"   // The new name for the competition (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /comps/comp123/name {"new_name": "Renamed Competition"}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Content Manager`

---

### 3.3. `PATCH /comps/<comp_id>/description` - Update Competition Description

*   **Description:** Updates the description of an existing competition.
*   **Request Method:** `PATCH`
*   **Endpoint:** `/comps/<comp_id>/description`
*   **Path Parameter:**
    *   `comp_id`: ID of the competition to update.
*   **Request Body (JSON):**

    ```json
    {
      "new_description": "string" // The new description for the competition (required)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api patch /comps/comp123/description {"new_description": "Updated competition description"}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Content Manager`

---

### 3.4. `DELETE /comps/<comp_id>` - Delete Competition

*   **Description:** Deletes a competition entry.
*   **Request Method:** `DELETE`
*   **Endpoint:** `/comps/<comp_id>`
*   **Path Parameter:**
    *   `comp_id`: ID of the competition to delete.

*   **Example Request (Discord Bot):**

    ```
    !api delete /comps/comp123
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Content Manager`

---

## 4. Build Endpoints (`/builds`)

### 4.1. `PUT /builds/` - Create Build

*   **Description:** Creates a new build entry.
*   **Request Method:** `PUT`
*   **Endpoint:** `/builds/`
*   **Request Body (JSON):**

    ```json
    {
      "weapon": "string",       // Main weapon of the build (required)
      "role": "string",         // Role of the build (e.g., DPS, Tank, Healer) (required)
      "cape": "string",         // Cape for the build (required)
      "...": "..."              // Other build components (off-hand, helmet, armor, boots, food, potion, mount)
    }
    ```

*   **Example Request (Discord Bot):**

    ```
    !api put /builds/ {"weapon": "Great Axe", "role": "DPS", "cape": "Lymhurst Cape"}
    ```

*   **Authorized Roles:** `Council`, `Officer`, `Content Manager`, `Member`






4.2. Update Build Weapon (PATCH /builds/<build_id>/weapon)
 * Description: Updates the weapon of an existing build.
 * Request Body:
   {
  "new_weapon": "string"
}

   * new_weapon: The new weapon for the build.
 * Example Command (Discord Bot):
   !api patch /builds/build123/weapon {"new_weapon": "Halberd"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.3. Update Build Off-Hand (PATCH /builds/<build_id>/off_hand)
 * Description: Updates the off-hand item of an existing build.
 * Request Body:
   {
  "new_off_hand": "string"
}

   * new_off_hand: The new off-hand item.
 * Example Command (Discord Bot):
   !api patch /builds/build123/off_hand {"new_off_hand": "Shield"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.4. Update Build Helmet (PATCH /builds/<build_id>/helmet)
 * Description: Updates the helmet of an existing build.
 * Request Body:
   {
  "new_helmet": "string"
}

   * new_helmet: The new helmet.
 * Example Command (Discord Bot):
   !api patch /builds/build123/helmet {"new_helmet": "Soldier Helmet"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.5. Update Build Helmet Chariot (PATCH /builds/<build_id>/helmet_chariot)
 * Description: Updates the chariot helmet of an existing build.
 * Request Body:
   {
  "new_helmet_chariot": "string"
}

   * new_helmet_chariot: The new chariot helmet.
 * Example Command (Discord Bot):
   !api patch /builds/build123/helmet_chariot {"new_helmet_chariot": "Royal Helmet"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.6. Update Build Armor (PATCH /builds/<build_id>/armor)
 * Description: Updates the armor of an existing build.
 * Request Body:
   {
  "new_armor": "string"
}

   * new_armor: The new armor.
 * Example Command (Discord Bot):
   !api patch /builds/build123/armor {"new_armor": "Mercenary Jacket"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.7. Update Build Boots (PATCH /builds/<build_id>/boots)
 * Description: Updates the boots of an existing build.
 * Request Body:
   {
  "new_boots": "string"
}

   * new_boots: The new boots.
 * Example Command (Discord Bot):
   !api patch /builds/build123/boots {"new_boots": "Soldier Boots"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.8. Update Build Food (PATCH /builds/<build_id>/food)
 * Description: Updates the food for an existing build.
 * Request Body:
   {
  "new_food": "string"
}

   * new_food: The new food.
 * Example Command (Discord Bot):
   !api patch /builds/build123/food {"new_food": "Omelette"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.9. Update Build Food Chariot (PATCH /builds/<build_id>/food_chariot)
 * Description: Updates the chariot food for an existing build.
 * Request Body:
   {
  "new_food_chariot": "string"
}

   * new_food_chariot: The new chariot food.
 * Example Command (Discord Bot):
   !api patch /builds/build123/food_chariot {"new_food_chariot": "Roast Pork"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.10. Update Build Potion (PATCH /builds/<build_id>/potion)
 * Description: Updates the potion for an existing build.
 * Request Body:
   {
  "new_potion": "string"
}

   * new_potion: The new potion.
 * Example Command (Discord Bot):
   !api patch /builds/build123/potion {"new_potion": "Healing Potion"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.11. Update Build Mount (PATCH /builds/<build_id>/mount)
 * Description: Updates the mount for an existing build.
 * Request Body:
   {
  "new_mount": "string"
}

   * new_mount: The new mount.
 * Example Command (Discord Bot):
   !api patch /builds/build123/mount {"new_mount": "Direwolf"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.12. Update Build Role (PATCH /builds/<build_id>/role)
 * Description: Updates the role of an existing build.
 * Request Body:
   {
  "new_role": "string"
}

   * new_role: The new role (e.g., DPS, Tank, Healer).
 * Example Command (Discord Bot):
   !api patch /builds/build123/role {"new_role": "Tank"}

 * Authorized Roles: Council, Officer, Content Manager, Member
4.13. Delete Build (DELETE /builds/<build_id>)
 * Description: Deletes a build entry permanently from the database.
 * Example Command (Discord Bot):
   !api delete /builds/build123

 * Authorized Roles: Council, Officer, Content Manager
5. Content Endpoints (/contents)
5.1. Create Content (PUT /contents/)
 * Description: Creates a new content entry.
 * Request Body:
   {
  "title": "string",
  "host_user_id": "string",
  "date": "string (YYYY-MM-DD)",
  "time": "string (HH:MM)",
  "comp_id": "string",
  "...": "..."
}

   * title: Title of the content.
   * host_user_id: User ID of the content host.
   * date: Date of the content.
   * time: Time of the content.
   * comp_id: Competition ID (if applicable).
   * ...: Other relevant content information.
 * Example Command (Discord Bot):
   !api put /contents/ {"title": "GvG Night", "host_user_id": "user456", "date": "2025-03-10", "time": "20:00", "comp_id": "comp789"}

 * Authorized Roles: Council, Officer, Content Manager
5.2. Update Content Title (PATCH /contents/<content_id>/title)
 * Description: Updates the title of an existing content entry.
 * Request Body:
   {
  "new_title": "string"
}

   * new_title: The new title for the content.
 * Example Command (Discord Bot):
   !api patch /contents/content123/title {"new_title": "Updated GvG Night Title"}

 * Authorized Roles: Council, Officer, Content Manager
5.3. Update Content Time (PATCH /contents/<content_id>/time)
 * Description: Updates the time of an existing content entry.
 * Request Body:
   {
  "new_time": "string (HH:MM)"
}

   * new_time: The new time for the content.
 * Example Command (Discord Bot):
   !api patch /contents/content123/time {"new_time": "21:00"}

 * Authorized Roles: Council, Officer, Content Manager
5.4. Update Content Date (PATCH /contents/<content_id>/date)
 * Description: Updates the date of an existing content entry.
 * Request Body:
   {
  "new_date": "string (YYYY-MM-DD)"
}

   * new_date: The new date for the content.
 * Example Command (Discord Bot):
   !api patch /contents/content123/date {"new_date": "2025-03-11"}

 * Authorized Roles: Council, Officer, Content Manager
5.5. Update Content Competition ID (PATCH /contents/<content_id>/comp_id)
 * Description: Updates the competition ID associated with a content entry.
 * Request Body:
   {
  "new_comp_id": "string"
}

   * new_comp_id: The new competition ID.
 * Example Command (Discord Bot):
   !api patch /contents/content123/comp_id {"new_comp_id": "comp456"}

 * Authorized Roles: Council, Officer, Content Manager
5.6. Delete Content (Cancellation) (DELETE /contents/<content_id>)
 * Description: Marks a content entry as canceled (does not permanently delete).
 * Note:  The content is not removed from the database but marked as canceled=true for statistical purposes.
 * Example Command (Discord Bot):
   !api delete /contents/content123

 * Authorized Roles: Council, Officer, Content Manager
6. Participant Endpoints (/partecipants)
6.1. Create Participant (PUT /partecipants/)
 * Description: Creates a new participant entry.
 * Request Body:
   {
  "user_id": "string",
  "build_id": "string"
}

   * user_id: User ID of the participant.
   * build_id: Build ID used by the participant.
 * Example Command (Discord Bot):
   !api put /partecipants/ {"user_id": "user789", "build_id": "build456"}

 * Authorized Roles: Council, Officer, Content Manager, Member, Guest
6.2. Update Participant Build (PATCH /partecipants/<partecipant_id>/build)
 * Description: Updates the build ID for a participant entry.
 * Request Body:
   {
  "new_build_id": "string"
}

   * new_build_id: The new build ID.
 * Example Command (Discord Bot):
   !api patch /partecipants/participant123/build {"new_build_id": "build789"}

 * Authorized Roles: Council, Officer, Content Manager, Member
6.3. Delete Participant (Disenrollment) (DELETE /partecipants/<partecipant_id>)
 * Description: Marks a participant entry as unchecked (disenrolled, does not permanently delete).
 * Note: The participant is not removed from the database but marked as check=false for statistical purposes.
 * Example Command (Discord Bot):
   !api delete /partecipants/participant123

 * Authorized Roles: Council, Officer, Content Manager, Member
7. Leaderboard Endpoints (/leaderboards)
7.1. Get Balance Leaderboard (GET /leaderboards/<discord_id>/balance)
 * Description: Retrieves the balance leaderboard for a Discord server.
 * Query Parameters:
   * limit: (Optional) Maximum number of entries to return (default: 50, can be customized).
   * linked: (Optional) Filter by linked users (true for linked users, false for unlinked users, default: true).
 * Example Command (Discord Bot):
   !api get /leaderboards/1234567890/balance?limit=25&linked=false

 * Authorized Roles: Council, Officer, Balance Manager, Member, Guest
7.2. Get Points Leaderboard (GET /leaderboards/<discord_id>/points)
 * Description: Retrieves the points leaderboard for a Discord server.
 * Query Parameters:
   * limit: (Optional) Maximum number of entries to return (default: 50, can be customized).
   * linked: (Optional) Filter by linked users (true for linked users, false for unlinked users, default: true).
 * Example Command (Discord Bot):
   !api get /leaderboards/1234567890/points?limit=100&linked=true

 * Authorized Roles: Council, Officer, Points Manager, Member, Guest
7.3. Get Attendance Leaderboard (GET /leaderboards/<discord_id>/attendance)
 * Description: Retrieves the attendance leaderboard for a Discord server.
 * Query Parameters:
   * limit: (Optional) Maximum number of entries to return (default: 50, can be customized).
   * linked: (Optional) Filter by linked users (true for linked users, false for unlinked users, default: true).
 * Example Command (Discord Bot):
   !api get /leaderboards/1234567890/points?limit=100&linked=true

 * Authorized Roles: Council, Officer, Points Manager, Member, Guest
7.3. Get Attendance Leaderboard (GET /leaderboards/<discord_id>/attendance)
 * Description: Retrieves the attendance leaderboard for a Discord server.
 * Query Parameters:
   * limit: (Optional) Maximum number of entries to return (default: 50, can be customized).
   * linked: (Optional) Filter by linked users (true for linked users, false for unlinked users, default: true).
 * Example Command (Discord Bot):
   !api get /leaderboards/1234567890/attendance?limit=50

 * Authorized Roles: Council, Officer, Points Manager, Content Manager, Member, Guest
7.4. Get VOD Review Leaderboard (GET /leaderboards/<discord_id>/vod)
 * Description: Retrieves the VOD review points leaderboard for a Discord server.
 * Query Parameters:
   * limit: (Optional) Maximum number of entries to return (default: 50, can be customized).
   * linked: (Optional) Filter by linked users (true for linked users, false for unlinked users, default: true).
 * Example Command (Discord Bot):
   !api get /leaderboards/1234567890/vod?limit=50

 * Authorized Roles: Council, Officer, Vod Review, Points Manager, Member, Guest
7.5. Get Donation Leaderboard (GET /leaderboards/<discord_id>/donation)
 * Description: Retrieves the donation points leaderboard for a Discord server.
 * Query Parameters:
   * limit: (Optional) Maximum number of entries to return (default: 50, can be customized).
   * linked: (Optional) Filter by linked users (true for linked users, false for unlinked users, default: true).
 * Example Command (Discord Bot):
   !api get /leaderboards/1234567890/donation?limit=50