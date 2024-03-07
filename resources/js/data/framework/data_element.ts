import { ObjectDataProperty } from "./data_property"
import { DatapackFormat } from "../../util/version_util"
import { validateResourceLocation } from "../../util/resource_location_util"
import { Serializable } from "./serializables"

export abstract class DataElement extends Serializable {
    name: string
    baseProperty: ObjectDataProperty

    constructor(category: string, name: string, populateProperties: (element: DataElement) => void) {
        super(category)
        this.name = name
        populateProperties(this)
    }

    serialize(format: DatapackFormat): any {
        return this.baseProperty.serialize(format)
    }

    abstract deserialize(json:object): void
}

export abstract class NamespacedDataElement extends DataElement {
    resourceLocation: string

    constructor(category:string, resourceLocation: string, displayName: string, populateProperties: (element: DataElement) => void) {
        super(category, displayName, populateProperties)

        if (validateResourceLocation(resourceLocation)) {
            this.resourceLocation = resourceLocation
        }
        else {
            throw new Error("Invalid format for resource location " + resourceLocation)
        }
    }
}