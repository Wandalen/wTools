# Task 007: Universal Entity-Component System

## 🎯 **Objective**

Create a generic entity-component composition system that works with any ECS framework, game engine, or entity management system through universal traits and adapters.

## 📋 **Current State**

Manual entity composition with framework-specific boilerplate:
```rust
// Different approaches for each framework
// Bevy
fn spawn_bevy_player(mut commands: Commands) 
{
  commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Player { health: 100.0 },
    Sprite::default(),
  ));
}

// Legion  
fn spawn_legion_player(world: &mut Legion::World) 
{
  world.push((
    Position { x: 0.0, y: 0.0 },
    Health { value: 100.0 },
    Renderable { sprite_id: 42 },
  ));
}

// Custom ECS
fn spawn_custom_entity(world: &mut MyWorld) 
{
  let entity = world.create_entity();
  world.add_component(entity, PositionComponent::new(0.0, 0.0));
  world.add_component(entity, HealthComponent::new(100.0));
  world.add_component(entity, RenderComponent::new("sprite.png"));
}
```

## 🎯 **Target State**

Universal entity composition that works with any system:
```rust
#[derive(EntityCompose)]
struct GameEntity 
{
  #[component(category = "transform")]
  position: Vec3,
  
  #[component(category = "gameplay")]
  health: f32,
  
  #[component(category = "rendering")]
  sprite: SpriteData,
  
  #[component(category = "physics")]
  rigidbody: RigidBodyData,
  
  #[component(custom = "setup_audio_source")]
  audio: AudioData,
}

// Same entity works with ANY ECS framework
let entity = GameEntity::default()
  .impute(Vec3::new(100.0, 200.0, 0.0))
  .impute(100.0f32)
  .impute(SpriteData::new("hero.png"))
  .impute(RigidBodyData::dynamic());

// Works with Bevy
let bevy_entity = entity.spawn_into(BevyAdapter, &mut bevy_world);

// Works with Legion
let legion_entity = entity.spawn_into(LegionAdapter, &mut legion_world);

// Works with custom ECS
let custom_entity = entity.spawn_into(MyEcsAdapter::new(), &mut my_world);

// Works with non-ECS systems (Unity-style, Godot-style, etc.)
let object = entity.spawn_into(GameObjectAdapter, &mut scene);
```

## 📝 **Detailed Requirements**

### **Core Universal Traits**

#### **EntityCompose Trait**
```rust
pub trait EntityCompose<A: EntityAdapter> {
  type EntityId;
  type Error;
  
  fn spawn_into(self, adapter: A, context: &mut A::Context) -> Result<Self::EntityId, Self::Error>;
  fn update_in(self, adapter: A, context: &mut A::Context, entity: Self::EntityId) -> Result<(), Self::Error>;
  fn remove_from(adapter: A, context: &mut A::Context, entity: Self::EntityId) -> Result<(), Self::Error>;
}

pub trait EntityAdapter {
  type Context;
  type EntityId;
  type Error: std::error::Error;
  
  fn spawn_entity<T>(&self, entity: T, context: &mut Self::Context) -> Result<Self::EntityId, Self::Error>
  where 
    T: IntoComponents<Self>;
    
  fn supports_component_type(&self, component_type: ComponentTypeId) -> bool;
}

pub trait IntoComponents<A: EntityAdapter> {
  fn into_components(self) -> Vec<ComponentData>;
  fn component_categories(&self) -> Vec<&'static str>;
}
```

#### **Generic Component Specification**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentSpec 
{
  pub category: ComponentCategory,
  pub metadata: ComponentMetadata,
  pub spawn_strategy: SpawnStrategy,
  pub update_behavior: UpdateBehavior,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentCategory 
{
  Transform,      // Position, rotation, scale
  Physics,        // Rigidbody, collider, physics material
  Rendering,      // Sprite, mesh, material, shader
  Audio,         // Audio source, listener, effects
  Gameplay,      // Health, score, player data
  AI,           // Behavior, state machine, pathfinding
  Custom(String), // User-defined categories
}

#[derive(Debug, Clone)]
pub struct ComponentMetadata 
{
  pub name: String,
  pub description: Option<String>,
  pub version: Option<String>,
  pub dependencies: Vec<ComponentCategory>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpawnStrategy 
{
  Required,       // Must be present when spawning
  Optional,       // Can be added later
  Lazy,          // Created on first access
  Computed,      // Derived from other components
}
```

### **Universal Adapter System**

#### **Bevy Adapter**
```rust
pub struct BevyAdapter;

impl EntityAdapter for BevyAdapter 
{
  type Context = bevy::ecs::world::World;
  type EntityId = bevy::ecs::entity::Entity;
  type Error = BevyEntityError;
  
  fn spawn_entity<T>(&self, entity: T, world: &mut Self::Context) -> Result<Self::EntityId, Self::Error>
  where 
    T: IntoComponents<Self>,
  {
    let components = entity.into_components();
    let mut entity_commands = world.spawn_empty();
    
    for component in components {
      match component.category {
        ComponentCategory::Transform => {
          if let Ok(transform) = component.data.downcast::<Transform>() {
            entity_commands.insert(*transform);
          }
        },
        ComponentCategory::Rendering => {
          if let Ok(sprite) = component.data.downcast::<Sprite>() {
            entity_commands.insert(*sprite);
          }
        },
        ComponentCategory::Physics => {
          if let Ok(rigidbody) = component.data.downcast::<RigidBody>() {
            entity_commands.insert(*rigidbody);
          }
        },
        ComponentCategory::Custom(name) => {
          // Handle custom component types
          self.spawn_custom_component(&mut entity_commands, &name, component.data)?;
        },
        _ => {
          // Handle other standard categories
        }
      }
    }
    
    Ok(entity_commands.id())
  }
  
  fn supports_component_type(&self, component_type: ComponentTypeId) -> bool 
{
    // Check if Bevy supports this component type
    matches!(component_type.category, 
      ComponentCategory::Transform |
      ComponentCategory::Rendering |
      ComponentCategory::Physics |
      ComponentCategory::Audio
    )
  }
}
```

#### **Legion Adapter**
```rust
pub struct LegionAdapter;

impl EntityAdapter for LegionAdapter 
{
  type Context = legion::World;
  type EntityId = legion::Entity;
  type Error = LegionEntityError;
  
  fn spawn_entity<T>(&self, entity: T, world: &mut Self::Context) -> Result<Self::EntityId, Self::Error>
  where 
    T: IntoComponents<Self>,
  {
    let components = entity.into_components();
    let mut component_tuple = ();
    
    // Legion requires compile-time known component tuples
    // This is more complex and might need macro assistance
    for component in components {
      // Convert to Legion-compatible format
      match component.category {
        ComponentCategory::Transform => {
          // Add to tuple or use Legion's dynamic component system
        },
        _ => {}
      }
    }
    
    Ok(world.push(component_tuple))
  }
}
```

#### **Custom ECS Adapter**
```rust
pub struct CustomEcsAdapter<W> 
{
  phantom: PhantomData<W>,
}

impl<W: CustomWorld> EntityAdapter for CustomEcsAdapter<W> {
  type Context = W;
  type EntityId = W::EntityId;
  type Error = CustomEcsError;
  
  fn spawn_entity<T>(&self, entity: T, world: &mut Self::Context) -> Result<Self::EntityId, Self::Error>
  where 
    T: IntoComponents<Self>,
  {
    let entity_id = world.create_entity();
    let components = entity.into_components();
    
    for component in components {
      // Use your custom ECS API
      world.add_component(entity_id, component.data)?;
    }
    
    Ok(entity_id)
  }
}

// Trait that custom ECS systems need to implement
pub trait CustomWorld {
  type EntityId: Copy;
  type ComponentData;
  
  fn create_entity(&mut self) -> Self::EntityId;
  fn add_component(&mut self, entity: Self::EntityId, component: Self::ComponentData) -> Result<(), CustomEcsError>;
  fn remove_component(&mut self, entity: Self::EntityId, component_type: ComponentTypeId) -> Result<(), CustomEcsError>;
}
```

#### **Game Object Adapter (Unity/Godot style)**
```rust
pub struct GameObjectAdapter;

impl EntityAdapter for GameObjectAdapter 
{
  type Context = Scene;
  type EntityId = GameObjectId;
  type Error = GameObjectError;
  
  fn spawn_entity<T>(&self, entity: T, scene: &mut Self::Context) -> Result<Self::EntityId, Self::Error>
  where 
    T: IntoComponents<Self>,
  {
    let game_object = scene.create_game_object();
    let components = entity.into_components();
    
    for component in components {
      match component.category {
        ComponentCategory::Transform => {
          game_object.add_component(TransformComponent::from(component.data));
        },
        ComponentCategory::Rendering => {
          game_object.add_component(RendererComponent::from(component.data));
        },
        ComponentCategory::Custom(name) => {
          // Add custom component by name
          game_object.add_component_by_name(&name, component.data);
        },
        _ => {}
      }
    }
    
    Ok(game_object.id())
  }
}

### **Universal Usage Patterns**

#### **Basic Entity Composition**
```rust
#[derive(EntityCompose)]
struct Player {
  #[component(category = "transform")]
  position: Vec3,
  
  #[component(category = "gameplay")]
  health: f32,
  
  #[component(category = "rendering")]  
  sprite: SpriteData,
}

// Works with any system through adapters
let player = Player::default()
  .impute(Vec3::new(0.0, 0.0, 0.0))
  .impute(100.0f32)
  .impute(SpriteData::from_file("player.png"));
```

#### **Cross-Platform Entity Definition**
```rust
#[derive(EntityCompose)]
struct UniversalEntity 
{
  #[component(category = "transform")]
  transform: TransformData,
  
  #[component(category = "physics", optional)]
  physics: Option<PhysicsData>,
  
  #[component(category = "custom", name = "ai_behavior")]
  ai: AIBehavior,
  
  #[component(category = "rendering", lazy)]
  rendering: RenderingData,
}

// Same entity works everywhere
let entity_data = UniversalEntity::default()
  .impute(TransformData::at(100.0, 200.0, 0.0))
  .impute(Some(PhysicsData::dynamic()))
  .impute(AIBehavior::player_controller());

// Spawn in different systems
let bevy_entity = entity_data.clone().spawn_into(BevyAdapter, &mut bevy_world)?;
let unity_object = entity_data.clone().spawn_into(UnityAdapter, &mut unity_scene)?;
let custom_entity = entity_data.spawn_into(MySystemAdapter, &mut my_world)?;
```

### **Asset Integration**

#### **Asset-Aware Entity Composition**
```rust
#[derive(EntityCompose)]
struct AssetEntity 
{
  #[component(
    category = "rendering",
    asset = "models/character.glb"
  )]
  model: ModelData,
  
  #[component(
    category = "audio",
    asset = "sounds/footsteps.ogg"
  )]
  audio: AudioData,
  
  #[component(
    category = "animation",
    asset = "animations/walk.anim"
  )]
  animation: AnimationData,
}

// Generic asset loading that works with any asset system
impl AssetEntity 
{
  pub async fn load_with<A: AssetLoader>(asset_loader: &A) -> Result<Self, A::Error> 
{
    let model = asset_loader.load_model("models/character.glb").await?;
    let audio = asset_loader.load_audio("sounds/footsteps.ogg").await?;
    let animation = asset_loader.load_animation("animations/walk.anim").await?;
    
    Ok(Self::default()
      .impute(ModelData::from(model))
      .impute(AudioData::from(audio))
      .impute(AnimationData::from(animation)))
  }
}

// Generic asset loader trait - works with any engine's asset system
pub trait AssetLoader {
  type Error;
  type ModelHandle;
  type AudioHandle;
  type AnimationHandle;
  
  async fn load_model(&self, path: &str) -> Result<Self::ModelHandle, Self::Error>;
  async fn load_audio(&self, path: &str) -> Result<Self::AudioHandle, Self::Error>;
  async fn load_animation(&self, path: &str) -> Result<Self::AnimationHandle, Self::Error>;
}
```

### **Event-Driven Component Updates**

#### **Event System Integration**
```rust
#[derive(EntityAssign)]
struct EventDrivenEntity 
{
  #[component(
    system = "health",
    events = ["DamageEvent", "HealEvent"]
  )]
  health: HealthComponent,
  
  #[component(
    system = "animation", 
    events = ["StateChangeEvent"],
    state_machine = "player_states"
  )]
  animator: AnimatorComponent,
}

// Generates event handlers
impl EventDrivenEntity 
{
  pub fn handle_damage_event(
    &mut self, 
    event: &DamageEvent
  ) -> Option<ComponentUpdate> {
    self.health.take_damage(event.amount);
    
    if self.health.is_dead() {
      Some(ComponentUpdate::Remove(ComponentType::Health))
    } else {
      Some(ComponentUpdate::Modified)
    }
  }
  
  pub fn register_event_handlers(event_bus: &mut EventBus) 
{
    event_bus.subscribe::<DamageEvent, Self>(Self::handle_damage_event);
    event_bus.subscribe::<HealEvent, Self>(Self::handle_heal_event);
  }
}
```

### **Query Generation and Optimization**

#### **Automatic Query Generation**
```rust
#[derive(EntityAssign)]
struct QueryableEntity 
{
  #[component(system = "movement", mutable)]
  position: Transform,
  
  #[component(system = "movement", read_only)]
  velocity: Velocity,
  
  #[component(system = "rendering", read_only)]
  sprite: SpriteComponent,
}

// Generates optimized queries
impl QueryableEntity 
{
  pub type MovementQuery = (&'static mut Transform, &'static Velocity);
  pub type RenderQuery = (&'static Transform, &'static SpriteComponent);
  
  pub fn movement_system(
    mut query: Query<Self::MovementQuery>
  ) {
    for (mut transform, velocity) in query.iter_mut() {
      transform.translation += velocity.linear * time.delta_seconds();
    }
  }
  
  pub fn render_system(
    query: Query<Self::RenderQuery>
  ) {
    for (transform, sprite) in query.iter() {
      render_sprite_at_position(sprite, transform.translation);
    }
  }
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_entity/` - New crate for universal entity composition
- `component_model_entity/src/lib.rs` - Core entity composition traits
- `component_model_entity/src/entity_derive.rs` - EntityCompose derive implementation
- `component_model_entity/src/spec.rs` - Component specifications and categories
- `component_model_entity/src/adapters/` - System adapter implementations
- `component_model_entity/src/adapters/bevy.rs` - Bevy ECS adapter
- `component_model_entity/src/adapters/legion.rs` - Legion ECS adapter
- `component_model_entity/src/adapters/custom.rs` - Custom ECS adapter trait
- `component_model_entity/src/adapters/gameobject.rs` - GameObject-style adapter
- `component_model_entity/src/assets.rs` - Generic asset loading integration
- `component_model_entity/src/errors.rs` - Universal error types
- `examples/universal_entity_example.rs` - Cross-platform entity examples
- `examples/entity_adapters/` - Specific adapter examples

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add entity dependency (feature-gated)

## ⚡ **Implementation Steps**

### **Phase 1: Core Generic System (Week 1-2)**
1. Create `component_model_entity` crate with universal traits
2. Implement `EntityCompose`, `EntityAdapter`, and `IntoComponents` traits
3. Create basic `EntityCompose` derive macro with component categories
4. Implement simple Bevy adapter as proof of concept
5. Basic testing infrastructure for generic system

### **Phase 2: Multi-System Adapters (Week 2-3)**
1. Implement Legion and custom ECS adapters
2. Add GameObject-style adapter for Unity/Godot patterns
3. Create generic asset loading integration
4. Cross-adapter compatibility testing

### **Phase 3: Advanced Universal Features (Week 3-4)**
1. Component dependency resolution and spawn strategies
2. Generic event system integration
3. Performance optimization across all adapters
4. Comprehensive documentation and examples
5. System-specific integration helpers

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  use bevy::prelude::*;
  
  #[test]
  fn test_entity_spawning() 
{
    #[derive(EntityAssign, Component)]
    struct TestEntity 
{
      #[component(system = "test")]
      value: i32,
    }
    
    let mut app = App::new();
    let entity = TestEntity::default()
      .impute(42)
      .spawn_in_bevy(&mut app.world.spawn());
      
    let component = app.world.get::<TestEntity>(entity).unwrap();
    assert_eq!(component.value, 42);
  }
  
  #[test]
  fn test_system_registration() 
{
    #[derive(EntityAssign)]
    struct TestEntity 
{
      #[component(system = "movement")]
      position: Vec3,
    }
    
    let mut app = App::new();
    TestEntity::register_systems(&mut app);
    
    // Verify system was added
    assert!(app.world.contains_resource::<SystemRegistry>());
  }
}
```

### **Integration Tests**
```rust
// tests/bevy_integration.rs
use bevy::prelude::*;
use component_model_ecs::*;

#[derive(EntityAssign, Component)]
struct Player 
{
  #[component(system = "movement")]
  position: Transform,
  
  #[component(system = "health")]
  health: f32,
}

#[test]
fn test_full_bevy_integration() 
{
  let mut app = App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, (movement_system, health_system));
    
  // Spawn player entity
  let player = Player::default()
    .impute(Transform::from_xyz(0.0, 0.0, 0.0))
    .impute(100.0f32);
    
  let entity = app.world.spawn(player).id();
  
  // Run one frame
  app.update();
  
  // Verify entity exists and components are correct
  let player_query = app.world.query::<(&Transform, &Player)>();
  let (transform, player) = player_query.get(&app.world, entity).unwrap();
  
  assert_eq!(transform.translation, Vec3::ZERO);
  assert_eq!(player.health, 100.0);
}

fn movement_system(mut query: Query<&mut Transform, With<Player>>) 
{
  // Movement logic
}

fn health_system(mut query: Query<&mut Player>) 
{
  // Health logic  
}
```

## 📊 **Success Metrics**

- [ ] **Universal Compatibility**: Works with ANY entity system through adapter pattern
- [ ] **System Agnostic**: Same entity definition works across ECS, GameObject, and custom systems
- [ ] **Extensible**: Easy to add new systems without changing core framework
- [ ] **Zero Lock-in**: Not tied to specific engines or ECS frameworks
- [ ] **95% Boilerplate Reduction**: Minimal entity composition code needed
- [ ] **Type Safety**: Compile-time validation of component compatibility
- [ ] **Performance**: Zero-cost abstractions, optimal generated code

## 🚧 **Potential Challenges**

1. **System Diversity**: Vast differences between ECS, GameObject, and custom systems
   - **Solution**: Flexible adapter pattern with extensible component categories

2. **Performance**: Additional abstraction layer overhead in game-critical code
   - **Solution**: Generate optimal code per adapter, extensive benchmarking

3. **Type Complexity**: Generic constraints across different entity systems
   - **Solution**: Incremental trait design with clear bounds

4. **Ecosystem Adoption**: Convincing game developers to adopt new patterns
   - **Solution**: Show clear migration benefits, provide compatibility layers

5. **Asset Integration**: Different engines have vastly different asset systems
   - **Solution**: Generic asset traits with engine-specific implementations

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute parsing infrastructure
  - Task 006 (Async Support) for async asset loading
- **Blocks**: None
- **Related**: 
  - Benefits from Task 002 (Popular Types) for common game types
  - Synergy with Task 005 (Universal Extraction) for similar adapter patterns

## 📅 **Timeline**

- **Week 1-2**: Core generic traits and basic Bevy adapter
- **Week 2-3**: Multi-system adapters and asset integration
- **Week 3-4**: Advanced features, optimization, and comprehensive testing

## 💡 **Future Enhancements**

- **Visual Scripting**: Generate node graphs from entity definitions universally
- **Hot Reloading**: Runtime entity modification across any system
- **Cross-Platform Serialization**: Save/load entities between different engines
- **Multiplayer Sync**: Network entity state synchronization universally
- **Debug Tools**: Universal entity inspection tools for any system
- **Performance Profiling**: Cross-platform entity performance analysis
- **Asset Pipelines**: Universal asset processing and optimization