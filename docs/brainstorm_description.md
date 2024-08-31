# Raconteur

Raconteur is a tool which allows writing short story modules, called "beats", without knowing the specifics ahead of time such as characters, locales, relationships, etc. Writers express constraints that must be satisfied for the story beat to function, and the "gaps" in those beats are filled at runtime by the characters/places/objects which satisfy the constraints.

## Structure

A story beat has the following structure:

- A list of aliases used within the beat. Similar to listing the cast of characters or "dramatis personae" in a script. Each alias is given a user defined type (ex: character, item, city...). Add an alias for every narrative object not known ahead of time you need for this beat.
- A set of constraints on the above aliases. When selecting story beats, constraints are evaluated in the current narration. If there exists entities for every alias that satisfy all the constraints, the beat's scenario can be played.
- A scenario. A graph of story nodes which contain conditions and instructions.
  - An instruction is a named bag of properties which get interpreted by the caller. It is a user defined DSL, a short of simple scripting language available to the writer. The simplest form of that DSL is mapping each instruction type to a function call. These instructions are the writers' mean of modifying the narrative, but those modifications must be applied by the caller to their own custom game logic.

## Typical workflow:

- Caller translates their current game world into a raconteur narration. - They send that narration as a query to raconteur.
- Raconteur finds story beats that are applicable to the current narration.
- User picks a beat from the returned list.
- User steps through the scenario, interpreting the instructions and updating the narration as they go.
- Repeat

## Things to note:

- Raconteur does not modify the narration. (Maybe writers could add props to entities? Could be useful to tag some characters/objets with custom props in order to facilitate writing beats that are meant to happen in a certain sequence. Although the point here is to keep beats modular.)
- The caller has many responsibilities: translating its game world into a narration whose format is defined by the user, choosing from available story beats (what means does he have to choose other than rng? Parsing instructions?) and translating instructions back to game logic.
- Raconteur relies on user defined concepts. It has no knowledge of what a "character" is, or any other story entity nor how they relate to each other. These are defined in a schema file by the user. There is no one size fits all solution to story telling, and each game structures its entities differently. The narration is a storywise abstraction of the game world, meant to convey the state of the game _narratively_ for writers to operate on.
- It is basically useless without a caller app managing its custom game world.
- All beats are authored. No proc gen or ai going on under the hood.

## Goals:

- Provide an easy method for writers to produce templated story pieces to be assembled in a procedural generation context. This is useful in a scenario where you might not know the characters' names, roles, relationships, etc. at the time of writing, but still need to write modular content that assumes the existence of certain characters/objects/etc. with certain traits.
- Provide a GUI editor for writers to easily edit schema, story beats and playtest them by editing a test narration. Ideally a plugin for a text editor (neovim? vscode? friggin msword? write my own?)
  - A graph editor to edit a beat's scenario
  - Autocompletion to suggest traits, aliases, global entities, etc.
  - A test window that lets you edit a narration and see available story beats
- Could be worthwhile to integrate a llm to generate beats that writers could then curate.

## Glossary

### Narration

The set of story entities, their properties and relationships.

### Entity

A story relevant "thing", like a character, an object, a place, a concept...

### Property

A labeled value. Can contain sub-properties.

### Relationship

Qualifies how one entity relates to another. Unidirectional. Can optionally contain a property (or should each possible value be a different relationships?).

### Beat

A modular piece of storytelling. Contains aliased entities, constraints and a scenario. Meant to be as small and general as possible to maximize composability.

### Alias

A name that refers to an entity within the context of the beat.

### Constraint

A condition the entity must fulfill to be bound to the corresponding alias.

### Instruction

A user defined labeled map of properties to be interpreted gameside.

### Scenario

A graph of scenario nodes.

### Scenario node

A node in the scenario graph. Contains the names of the following nodes, a list of additional constraints and a list of instructions.
