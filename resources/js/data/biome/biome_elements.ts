import { DataElement } from "../framework/data_element";
import { StandardDataProperty } from "../framework/data_property";
import { SerializableBoolean } from "../framework/serializables";

export class BiomeDataElement extends DataElement {
    constructor(name: string) {
        super(name, (element: DataElement) => {
            element.properties.addProperty(
                new StandardDataProperty<SerializableBoolean>(
                        SerializableBoolean,
                        SerializableBoolean.of(false),
                        "has_precipitation"))
        })
    }

    serialize(): object {
        throw new Error("Method not implemented.");
    }

    deserialize(json: object): void {
        throw new Error("Method not implemented.");
    }
}