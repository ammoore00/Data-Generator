import { DatapackFormat } from "../../util/version_util"
import { Serializable, SerializableArray } from "./serializables"

export abstract class DataProperty<T extends Serializable> {
    type: Serializable
    name?: string

    constructor(type: Serializable, name?: string) {
        this.name = name
        this.type = type
    }

    abstract getValue(): T
    abstract serialize(format: DatapackFormat): any
}

export class StandardDataProperty<T extends Serializable> extends DataProperty<T> {
    value: T

    constructor(type: Serializable, defaultValue: T, name?: string) {
        super(type, name)
        this.value = defaultValue
    }

    getValue(): T {
        return this.value
    }

    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name != "") {
            output = this.name + ":"
        }

        output += this.value.serialize(format)
        return output
    }
}

export class ObjectDataProperty extends DataProperty<SerializableArray<DataProperty<any>>> {
    properties: DataProperty<any>[]

    constructor(type: Serializable, name?: string) {
        super(type, name)
        this.properties = new Array<DataProperty<any>>
    }

    addProperty(property: DataProperty<any>): void {
        this.properties.push(property)
    }

    getValue(): SerializableArray<DataProperty<any>> {
        throw this.properties
    }

    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name) {
            output = this.name + ":"
        }

        output += "{\n"

        for (let property of this.properties) {
            if (property.name == "") {
                throw new Error("Sub-properties of object data property cannot be anonymous!")
            }
            
            output += property.serialize + ",\n"
        }

        output = output.slice(0, -1) // Remove the trailing comma
        output += "}"

        return output
    }
}

export class ArrayDataProperty<T extends Serializable> extends DataProperty<SerializableArray<T>> {
    entries: T[]

    constructor(type: Serializable, name?: string) {
        super(type, name)
        this.entries = new Array<T>
    }

    addEntry(entry: T): void {
        this.entries.push(entry)
    }

    getValue(): SerializableArray<T> {
        throw this.entries
    }

    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name != "") {
            output = this.name + ":"
        }

        output += "[\n"

        for (let entry of this.entries) {
            if (entry instanceof DataProperty && entry.name != "") {
                throw new Error("Sub-properties of array data property cannot be named!")
            }
    
            entry.serialize(format)
            output += this.serlializeEntry(entry, format) + ",\n"
        }

        output = output.slice(0, -1) // Remove the trailing comma
        output += "]"

        return output
    }

    serlializeEntry(entry: T, format: DatapackFormat): any {
        return "" + entry
    }
}

export class VersionedDataProperty extends DataProperty<any> {
    protected propertyMap: Map<(format: DatapackFormat) => boolean, DataProperty<any>>

    addProperty(condition: (format: DatapackFormat) => boolean, dataProperty: DataProperty<any>): void {
        this.propertyMap.set(condition, dataProperty)
    }

    getValue() {
        throw new Error("Method not implemented.")
    }

    serialize(format: DatapackFormat): any {
        for (let [condition, dataProperty] of this.propertyMap) {
            if (condition.apply(format)) {
                return dataProperty.serialize(format)
            }
        }

        throw new Error("Format " + format + " does not have any valid data properties assigned for " + this.name)
    }
}