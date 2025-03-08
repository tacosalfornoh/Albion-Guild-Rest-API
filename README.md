### Database:
- Surreal DB

### Todo List:
- discord_controller rimuovere .clone e usare i & riferimenti
- rimuovere .unwrap o .expect s sostituirli con Enum::Result

## API Documentation

This document outlines the API endpoints for managing Discord, User, Competitions, Builds, Contents, and Participants data.

### 1. Discord Endpoints (`/discords`)

#### 1.1. Create Discord Entry (`PUT /discords`)

- **Description:** Creates a new Discord entry.
- **Request Body:**
  ```json
  {
    "discord_id": "string",
    "discord_name": "string",
    "joined_at": "string",
    "...": "..."
  }

 * discord_id: Unique identifier for the Discord server.
 * discord_name: Name of the Discord server.
 * joined_at: Date and time when the bot joined the server.
 * ...:  Other relevant Discord information.
 * Example Command (Discord Bot):
   !api put /discords {"discord_id": "1234567890", "discord_name": "My Guild", "joined_at": "2024-03-08T19:30:00Z"}

 * Authorized Roles: Council, Officer
1.2. Update Discord Name (PATCH /discords/<discord_id>/name)
 * Description: Updates the name of an existing Discord entry.
 * Request Body:
   {
  "new_name": "string"
}

   * new_name: The new name for the Discord server.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/name {"new_name": "My Renamed Guild"}

 * Authorized Roles: Council, Officer
1.3. Update Discord Balance Feature Status (PATCH /discords/<discord_id>/balance)
 * Description: Enables or disables the balance feature for a Discord server.
 * Request Body:
   {
  "balance": "boolean"
}

   * balance: true to enable the balance feature, false to disable.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/balance {"balance": true}

 * Authorized Roles: Council, Officer
1.4. Get Discord Balance Feature Status (GET /discords/<discord_id>/balance)
 * Description: Retrieves the status of the balance feature for a Discord server.
 * Example Command (Discord Bot):
   !api get /discords/1234567890/balance

 * Authorized Roles: Council, Officer, Balance Manager
1.5. Update Discord Application Feature Status (PATCH /discords/<discord_id>/application)
 * Description: Enables or disables the application feature for a Discord server.
 * Request Body:
   {
  "application": "boolean"
}

   * application: true to enable the application feature, false to disable.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/application {"application": false}

 * Authorized Roles: Council, Officer, Application Manager
1.6. Update Discord Content Feature Status (PATCH /discords/<discord_id>/content)
 * Description: Enables or disables the content feature for a Discord server.
 * Request Body:
   {
  "content": "boolean"
}

   * content: true to enable the content feature, false to disable.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/content {"content": true}

 * Authorized Roles: Council, Officer, Content Manager
1.7. Update Discord Logs Feature Status (PATCH /discords/<discord_id>/logs)
 * Description: Enables or disables the logs feature for a Discord server.
 * Request Body:
   {
  "logs": "boolean"
}

   * logs: true to enable the logs feature, false to disable.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/logs {"logs": false}

 * Authorized Roles: Council, Officer
1.8. Update Discord Builds Feature Status (PATCH /discords/<discord_id>/builds)
 * Description: Enables or disables the builds feature for a Discord server.
 * Request Body:
   {
  "builds": "boolean"
}

   * builds: true to enable the builds feature, false to disable.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/builds {"builds": true}

 * Authorized Roles: Council, Officer, Content Manager
1.9. Update Discord Bot Left Status (PATCH /discords/<discord_id>/left)
 * Description: Updates the status indicating if the bot has left the Discord server.
 * Request Body:
   {
  "is_bot_left": "boolean"
}

   * is_bot_left: true if the bot has left, false otherwise.
 * Example Command (Discord Bot):
   !api patch /discords/1234567890/left {"is_bot_left": true}

 * Authorized Roles: Council, Officer
1.10. Get Discord Bot Left Status (GET /discords/<discord_id>/left)
 * Description: Retrieves the status indicating if the bot has left the Discord server.
 * Example Command (Discord Bot):
   !api get /discords/1234567890/left

 * Authorized Roles: Council, Officer
2. User Endpoints (/users)
2.1. Create User Entry (PUT /users)
 * Description: Creates a new user entry.
 * Request Body:
   {
  "user_id": "string",
  "username": "string",
  "balance": "integer",
  "...": "..."
}

   * user_id: Unique identifier for the user.
   * username: Username of the user.
   * balance: User's initial balance.
   * ...: Other relevant user information.
 * Example Command (Discord Bot):
   !api put /users {"user_id": "user123", "username": "ExampleUser", "balance": 100}

 * Authorized Roles: Council, Officer, Balance Manager, Points Manager
2.2. Update Username (PATCH /users/<user_id>/username)
 * Description: Updates the username of an existing user.
 * Request Body:
   {
  "new_username": "string"
}

   * new_username: The new username.
 * Example Command (Discord Bot):
   !api patch /users/user123/username {"new_username": "NewUsername"}

 * Authorized Roles: Council, Officer, Points Manager
2.3. Update VOD Points (PATCH /users/<user_id>/vod)
 * Description: Updates the VOD points for a user.
 * Request Body:
   {
  "points": "integer"
}

   * points: Points to add or set for VOD review.
 * Example Command (Discord Bot):
   !api patch /users/user123/vod {"points": 5}

 * Authorized Roles: Council, Officer, Vod Review, Points Manager
2.4. Link In-Game Name (PATCH /users/<user_id>/link)
 * Description: Links an in-game name to a user.
 * Request Body:
   {
  "ign_name": "string"
}

   * ign_name: In-game name to link.
 * Example Command (Discord Bot):
   !api patch /users/user123/link {"ign_name": "ExampleIGN"}

 * Authorized Roles: Council, Officer, Application Manager, Points Manager
2.5. Update Attendance Points (PATCH /users/<user_id>/attendance)
 * Description: Updates the attendance points for a user.
 * Request Body:
   {
  "points": "integer"
}

   * points: Points to add or set for attendance.
 * Example Command (Discord Bot):
   !api patch /users/user123/attendance {"points": 10}

 * Authorized Roles: Council, Officer, Points Manager, Content Manager
2.6. Update Balance (PATCH /users/<user_id>/balance)
 * Description: Updates the balance of a user.
 * Request Body:
   {
  "amount": "integer"
}

   * amount: Amount to add or set to the user's balance.
 * Example Command (Discord Bot):
   !api patch /users/user123/balance {"amount": 50}

 * Authorized Roles: Council, Officer, Balance Manager
2.7. Remove In-Game Name Link (DELETE /users/<user_id>/link)
 * Description: Removes the linked in-game name from a user.
 * Example Command (Discord Bot):
   !api delete /users/user123/link

 * Authorized Roles: Council, Officer, Application Manager, Points Manager
2.8. Get Balance (GET /users/<user_id>/balance)
 * Description: Retrieves the balance of a user.
 * Example Command (Discord Bot):
   !api get /users/user123/balance

 * Authorized Roles: Council, Officer, Balance Manager, Member, Guest
2.9. Update User Bot Left Status (PATCH /users/<user_id>/left)
 * Description: Updates the status indicating if the bot has left the user's context (e.g., Discord server).
 * Request Body:
   {
  "is_bot_left": "boolean"
}

   * is_bot_left: true if the bot has left, false otherwise.
 * Example Command (Discord Bot):
   !api patch /users/user123/left {"is_bot_left": true}

 * Authorized Roles: Council, Officer
2.10. Get User Bot Left Status (GET /users/<user_id>/left)
 * Description: Retrieves the status indicating if the bot has left the user's context.
 * Example Command (Discord Bot):
   !api get /users/user123/left

 * Authorized Roles: Council, Officer
2.11. Update Donation Points (PATCH /users/<user_id>/donation)
 * Description: Updates the donation points for a user.
 * Request Body:
   {
  "points": "integer"
}

   * points: Points to add or set for donation tracking.
 * Example Command (Discord Bot):
   !api patch /users/user123/donation {"points": 20}

 * Authorized Roles: Council, Officer, Points Manager
3. Competition Endpoints (/comps)
3.1. Create Competition (PUT /comps)
 * Description: Creates a new competition entry.
 * Request Body:
   {
  "name": "string",
  "content": "string",
  "description": "string"
}

   * name: Name of the competition.
   * content: Content type of the competition.
   * description: Description of the competition.
 * Example Command (Discord Bot):
   !api put /comps {"name": "Build Competition", "content": "Open World PvP", "description": "Best open world build competition"}

 * Authorized Roles: Council, Officer, Content Manager
3.2. Update Competition Name (PATCH /comps/<comp_id>/name)
 * Description: Updates the name of an existing competition.
 * Request Body:
   {
  "new_name": "string"
}

   * new_name: The new name for the competition.
 * Example Command (Discord Bot):
   !api patch /comps/comp123/name {"new_name": "Renamed Competition"}

 * Authorized Roles: Council, Officer, Content Manager
3.3. Update Competition Description (PATCH /comps/<comp_id>/description)
 * Description: Updates the description of an existing competition.
 * Request Body:
   {
  "new_description": "string"
}

   * new_description: The new description for the competition.
 * Example Command (Discord Bot):
   !api patch /comps/comp123/description {"new_description": "Updated competition description"}

 * Authorized Roles: Council, Officer, Content Manager
3.4. Delete Competition (DELETE /comps/<comp_id>)
 * Description: Deletes a competition entry.
 * Example Command (Discord Bot):
   !api delete /comps/comp123

 * Authorized Roles: Council, Officer, Content Manager
4. Build Endpoints (/builds)
4.1. Create Build (PUT /builds/)
 * Description: Creates a new build entry.
 * Request Body:
   {
  "weapon": "string",
  "role": "string",
  "cape": "string",
  "...": "..."
}

   * weapon: Main weapon of the build.
   * role: Role of the build (e.g., DPS, Tank, Healer).
   * cape: Cape for the build.
   * ...: Other build components (off-hand, helmet, armor, boots, food, potion, mount).
 * Example Command (Discord Bot):
   !api put /builds/ {"weapon": "Great Axe", "role": "DPS", "cape": "Lymhurst Cape"}

 * Authorized Roles: Council, Officer, Content Manager, Member
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
 * Description: Retrieves the attendance leaderboard for a