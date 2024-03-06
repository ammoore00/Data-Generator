import { ObjectDataProperty } from "./data_property"
import { DatapackFormat } from "../../util/version_util"
import { validateResourceLocation } from "../../util/resource_location_util"

export abstract class DataElement {
    name: string
    properties: ObjectDataProperty

    constructor(name: string, populateProperties: (element: DataElement) => void) {
        this.name = name
        populateProperties(this)
    }

    abstract serialize(format: DatapackFormat): object
    abstract deserialize(json:object): void
}

export abstract class NamespacedDataElement extends DataElement {
    resourceLocation: string

    constructor(resourceLocation: string, displayName: string, populateProperties: (element: DataElement) => void) {
        super(displayName, populateProperties)

        if (validateResourceLocation(resourceLocation)) {
            this.resourceLocation = resourceLocation
        }
        else {
            throw new Error("Invalid format for resource location " + resourceLocation)
        }
    }
}