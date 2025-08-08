# Task 007: Game Development ECS Integration

## 🎯 **Objective**

Create specialized derives for Entity Component System (ECS) integration, enabling seamless component model usage in game development with popular ECS frameworks like Bevy, Legion, and Specs.

## 📋 **Current State**

Manual ECS component management:
```rust
// Bevy - manual component spawning
fn spawn_player(mut commands: Commands) {
  commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Player { health: 100.0 },
    Sprite::default(),
    AudioSource::new("footsteps.wav"),
  ));
}

// Manual component updates
fn update_player(mut query: Query<(&mut Transform, &mut Player)>) {
  for (mut transform, mut player) in query.iter_mut() {
    transform.translation.x += 1.0;
    player.health -= 0.1;
  }
}
```

## 🎯 **Target State**

Component model driven ECS:
```rust
#[derive(EntityAssign)]
struct Player {
  #[component(system = "physics")]
  position: Vec3,
  
  #[component(system = "rendering", asset = "sprites/player.png")]
  sprite: SpriteComponent,
  
  #[component(system = "audio", sound = "footsteps.wav")]
  audio: AudioComponent,
  
  #[component(system = "gameplay")]
  health: f32,
  
  #[component(system = "ai", behavior = "player_controller")]
  controller: PlayerController,
}

// Spawn entity with all components
let player = Player::default()
  .impute(Vec3::new(100.0, 200.0, 0.0))
  .impute(SpriteComponent::new("hero.png"))
  .impute(AudioComponent::new("walk.wav"))
  .impute(100.0f32)
  .impute(PlayerController::new());

let entity_id = world.spawn_entity(player);

// Systems automatically process based on component registration
physics_system.update(&mut world);  // Processes position
render_system.update(&mut world);   // Processes sprite
audio_system.update(&mut world);    // Processes audio
```

## 📝 **Detailed Requirements**

### **Core ECS Traits**

#### **EntityAssign Trait**
```rust
pub trait EntityAssign {
  type EntityId;
  type World;
  
  fn spawn_in_world(self, world: &mut Self::World) -> Self::EntityId;
  fn despawn_from_world(world: &mut Self::World, entity: Self::EntityId);
  fn sync_from_world(world: &Self::World, entity: Self::EntityId) -> Option<Self>
  where 
    Self: Sized;
}

pub trait SystemComponent {
  fn system_name() -> &'static str;
  fn component_types() -> Vec<ComponentTypeId>;
}
```

#### **ComponentSystem Integration**
```rust
pub trait ComponentSystem<W: World> {
  type ComponentQuery;
  
  fn query_components(world: &W) -> Self::ComponentQuery;
  fn process_entity(world: &mut W, entity: EntityId);
  fn process_all_entities(world: &mut W);
}
```

### **ECS Framework Integration**

#### **Bevy Integration**
```rust
#[derive(EntityAssign)]
#[entity(framework = "bevy")]
struct GameEntity {
  #[component(system = "transform")]
  position: Transform,
  
  #[component(system = "rendering")]
  sprite: Sprite,
  
  #[component(system = "physics")]
  rigidbody: RigidBody,
}

// Generates Bevy Bundle implementation
impl Bundle for GameEntity {
  type Components = (Transform, Sprite, RigidBody);
  
  fn components(self) -> Self::Components {
    (self.position, self.sprite, self.rigidbody)
  }
}

// Generates spawning methods
impl GameEntity {
  pub fn spawn_in_bevy(self, commands: &mut Commands) -> Entity {
    commands.spawn(self).id()
  }
  
  pub fn spawn_with_children<F>(
    self, 
    commands: &mut Commands, 
    children: F
  ) -> Entity 
  where
    F: FnOnce(&mut ChildBuilder),
  {
    commands.spawn(self).with_children(children).id()
  }
}

// System integration
impl IntoSystemConfigs<()> for GameEntity {
  fn into_configs(self) -> SystemConfigs {
    (
      transform_system,
      rendering_system, 
      physics_system,
    ).into_configs()
  }
}
```

#### **Legion Integration**
```rust
#[derive(EntityAssign)]
#[entity(framework = "legion")]
struct LegionEntity {
  #[component(archetype = "player")]
  player_stats: PlayerStats,
  
  #[component(archetype = "renderable")]
  mesh: MeshComponent,
}

// Generates Legion-specific code
impl LegionEntity {
  pub fn spawn_in_legion(self, world: &mut legion::World) -> legion::Entity {
    world.push((
      self.player_stats,
      self.mesh,
    ))
  }
  
  pub fn create_archetype() -> legion::systems::CommandBuffer {
    let mut cmd = legion::systems::CommandBuffer::new();
    cmd.push((PlayerStats::default(), MeshComponent::default()));
    cmd
  }
}
```

### **System Registration and Management**

#### **Automatic System Registration**
```rust
#[derive(EntityAssign)]
struct ComplexEntity {
  #[component(
    system = "physics", 
    update_order = "1",
    dependencies = ["input_system"]
  )]
  physics: PhysicsComponent,
  
  #[component(
    system = "rendering", 
    update_order = "2",
    dependencies = ["physics"]
  )]
  sprite: SpriteComponent,
  
  #[component(
    system = "audio",
    update_order = "1", 
    conditional = "audio_enabled"
  )]
  audio: AudioComponent,
}

// Generates system scheduling
impl ComplexEntity {
  pub fn register_systems<T: SystemScheduler>(scheduler: &mut T) {
    scheduler
      .add_system(physics_system.label("physics").after("input_system"))
      .add_system(rendering_system.label("rendering").after("physics"))
      .add_system(audio_system.label("audio").run_if(audio_enabled));
  }
}
```

### **Asset Loading Integration**

#### **Asset-Aware Components**
```rust
#[derive(EntityAssign)]
struct AssetEntity {
  #[component(
    system = "rendering",
    asset_path = "models/character.glb",
    asset_type = "Model"
  )]
  model: ModelComponent,
  
  #[component(
    system = "audio",
    asset_path = "sounds/footsteps.ogg",
    asset_type = "AudioClip"
  )]
  footstep_sound: AudioComponent,
  
  #[component(
    system = "animation",
    asset_path = "animations/walk.anim",
    asset_type = "AnimationClip"  
  )]
  walk_animation: AnimationComponent,
}

// Generates asset loading
impl AssetEntity {
  pub async fn load_assets(asset_server: &AssetServer) -> Self {
    let model = asset_server.load("models/character.glb").await;
    let sound = asset_server.load("sounds/footsteps.ogg").await;
    let animation = asset_server.load("animations/walk.anim").await;
    
    Self::default()
      .impute(ModelComponent::new(model))
      .impute(AudioComponent::new(sound))
      .impute(AnimationComponent::new(animation))
  }
}
```

### **Event-Driven Component Updates**

#### **Event System Integration**
```rust
#[derive(EntityAssign)]
struct EventDrivenEntity {
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
impl EventDrivenEntity {
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
  
  pub fn register_event_handlers(event_bus: &mut EventBus) {
    event_bus.subscribe::<DamageEvent, Self>(Self::handle_damage_event);
    event_bus.subscribe::<HealEvent, Self>(Self::handle_heal_event);
  }
}
```

### **Query Generation and Optimization**

#### **Automatic Query Generation**
```rust
#[derive(EntityAssign)]
struct QueryableEntity {
  #[component(system = "movement", mutable)]
  position: Transform,
  
  #[component(system = "movement", read_only)]
  velocity: Velocity,
  
  #[component(system = "rendering", read_only)]
  sprite: SpriteComponent,
}

// Generates optimized queries
impl QueryableEntity {
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
- `component_model_ecs/` - New crate for ECS integration
- `component_model_ecs/src/lib.rs` - Main ECS API
- `component_model_ecs/src/entity_derive.rs` - EntityAssign derive implementation
- `component_model_ecs/src/bevy.rs` - Bevy-specific implementations
- `component_model_ecs/src/legion.rs` - Legion integration
- `component_model_ecs/src/specs.rs` - Specs integration  
- `component_model_ecs/src/systems.rs` - System management utilities
- `component_model_ecs/src/assets.rs` - Asset loading integration
- `component_model_ecs/src/events.rs` - Event system integration
- `component_model_ecs/src/queries.rs` - Query generation
- `examples/ecs_game_example.rs` - Complete game example

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add ECS dependency (feature-gated)

## ⚡ **Implementation Steps**

### **Phase 1: Bevy Integration (Week 1-2)**
1. Create `component_model_ecs` crate with Bevy focus
2. Implement `EntityAssign` derive macro for Bevy Bundle generation
3. Add basic system registration and component spawning
4. Create asset loading integration
5. Basic testing with Bevy examples

### **Phase 2: Multi-Framework Support (Week 2-3)**
1. Add Legion and Specs support
2. Create framework-agnostic traits and abstractions
3. Implement cross-framework compatibility layer
4. Advanced query generation

### **Phase 3: Advanced Features (Week 3-4)**
1. Event system integration
2. Asset loading and dependency management
3. Performance optimization and benchmarking
4. State machine integration
5. Comprehensive documentation and examples

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  use bevy::prelude::*;
  
  #[test]
  fn test_entity_spawning() {
    #[derive(EntityAssign, Component)]
    struct TestEntity {
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
  fn test_system_registration() {
    #[derive(EntityAssign)]
    struct TestEntity {
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
struct Player {
  #[component(system = "movement")]
  position: Transform,
  
  #[component(system = "health")]
  health: f32,
}

#[test]
fn test_full_bevy_integration() {
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

fn movement_system(mut query: Query<&mut Transform, With<Player>>) {
  // Movement logic
}

fn health_system(mut query: Query<&mut Player>) {
  // Health logic  
}
```

## 📊 **Success Metrics**

- [ ] Support for 3+ major ECS frameworks (Bevy, Legion, Specs)
- [ ] Automatic system registration and scheduling
- [ ] Asset loading integration
- [ ] 90% reduction in ECS boilerplate code
- [ ] Performance equivalent to manual ECS usage
- [ ] Event-driven component updates

## 🚧 **Potential Challenges**

1. **Framework Differences**: Each ECS has different architecture
   - **Solution**: Abstract common patterns, framework-specific implementations

2. **Performance**: ECS systems need to be extremely fast
   - **Solution**: Generate optimal queries, avoid runtime overhead

3. **Type Safety**: Complex generic constraints across frameworks
   - **Solution**: Careful trait design and compile-time validation

4. **Asset Dependencies**: Complex asset loading graphs
   - **Solution**: Dependency resolution system and async loading

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute infrastructure
  - Task 006 (Async Support) for asset loading
- **Blocks**: None
- **Related**: Benefits from all other tasks for comprehensive game dev support

## 📅 **Timeline**

- **Week 1-2**: Bevy integration and core framework
- **Week 2-3**: Multi-framework support and abstractions
- **Week 3-4**: Advanced features, optimization, and documentation

## 💡 **Future Enhancements**

- **Visual Scripting**: Generate visual node graphs from component definitions
- **Hot Reloading**: Runtime component modification and system recompilation
- **Networking**: Synchronize components across network for multiplayer
- **Serialization**: Save/load entity states and component data
- **Debug Tools**: Runtime component inspection and modification tools
- **Performance Profiling**: Built-in profiling for component systems