import { DataCategories } from "../framework/data_categories";
import { DataElement, NamespacedDataElement } from "../framework/data_element";
import { ArrayDataProperty, ObjectDataProperty, StandardDataProperty } from "../framework/data_property";
import { SerializableBoolean, SerializableDouble, SerializableFloat, SerializableInt, SerializableResourceLocation, SerializableString } from "../framework/serializables";
import { CarverElement as CarverDataElement } from "./carver_element";
import { PlacedFeatureDataElement } from "./placed_feature_element";
import { SpawnerCategoryDataElement } from "./mob_spawning/spawner_category_element";
import { SpawnCostsDataElement } from "./mob_spawning/spawn_costs_element";

function generateBiomeProperties(element: DataElement) {
    element.baseProperty
    .addProperty(new StandardDataProperty<SerializableBoolean>(SerializableBoolean.of(false), "has_precipitation"))
    .addProperty(new StandardDataProperty<SerializableFloat>(SerializableFloat.of(0), "temperature"))
    .addProperty(new StandardDataProperty<SerializableString<"none"|"frozen">>(SerializableString.of<"none"|"frozen">("none"), "temperature_modifier").setOptional())
    .addProperty(new StandardDataProperty<SerializableFloat>(SerializableFloat.of(0), "downfall"))
    .addProperty(new ObjectDataProperty("effects")
        .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(12638463), "fog_color"))
        .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "sky_color"))
        .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(4159204), "water_color"))
        .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(329011), "water_fog_color"))
        .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "foliage_color").setOptional())
        .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "grass_color").setOptional())
        .addProperty(new StandardDataProperty<SerializableString<"none"|"dark_forest"|"swamp">>(SerializableString.of<"none"|"dark_forest"|"swamp">("none"), "grass_color_modifier").setOptional())
        .addProperty(new ParticleDataProperty("particle").setOptional())
        .addProperty(new SoundDataProperty("ambient_sound").setOptional())
        .addProperty((new ObjectDataProperty("mood_sound").setOptional() as ObjectDataProperty)
            .addProperty(new SoundDataProperty("sound"))
            .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "tick_delay"))
            .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "block_search_extent"))
            .addProperty(new StandardDataProperty<SerializableDouble>(SerializableDouble.of(0), "offset"))
        )
        .addProperty((new ObjectDataProperty("additions_sound").setOptional() as ObjectDataProperty)
            .addProperty(new SoundDataProperty("sound"))
            .addProperty(new StandardDataProperty<SerializableDouble>(SerializableDouble.of(0), "tick_chance"))
        )
        .addProperty((new ObjectDataProperty("music").setOptional() as ObjectDataProperty)
            .addProperty(new SoundDataProperty("sound"))
            .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "min_delay"))
            .addProperty(new StandardDataProperty<SerializableInt>(SerializableInt.of(0), "max_delay"))
            .addProperty(new StandardDataProperty<SerializableBoolean>(SerializableBoolean.of(false), "replace_current_music"))
        )
    )
    .addProperty(new ObjectDataProperty("carvers")
        .addProperty(new CarverDataProperty("air"))
        .addProperty(new CarverDataProperty("liquid"))
    )
    .addProperty(new FeatureDataProperty("carvers"))
    .addProperty(new StandardDataProperty<SerializableFloat>(SerializableFloat.of(0), "creature_spawn_probability").setOptional())
    .addProperty(new StandardDataProperty<SpawnerCategoryDataElement>("spawners"))
    .addProperty(new StandardDataProperty<SpawnCostsDataElement>("spawn_costs"))
}

export class BiomeDataElement extends NamespacedDataElement {
    constructor(resourceLocation: string, displayName: string) {
        super(DataCategories.DATA_ELEMENT.BIOME, resourceLocation, displayName, generateBiomeProperties)
    }

    deserialize(json: object): void {
        throw new Error("Method not implemented.");
    }
}

class SoundDataProperty extends ObjectDataProperty {
    constructor(name?: string) {
        super(name)

        this.addProperty(new StandardDataProperty<SerializableResourceLocation>(DataCategories.RESOURCE_LOCATION.SOUND))
        this.addProperty(new StandardDataProperty<SerializableFloat>(SerializableFloat.of(0), "range"))
    }
}

class ParticleDataProperty extends ObjectDataProperty {
    // Has special conditional properties based on the value of the "ty[e" field - see custom biome page on the wiki for more
}

class CarverDataProperty extends ArrayDataProperty<CarverDataElement> {
    // Needs special deserialization - see custom biome page on the wiki for more
}

class FeatureDataProperty extends ArrayDataProperty<PlacedFeatureDataElement> {
    // Contains a fixed number of entries in a specific order - see custom biome page on the wiki for more
}