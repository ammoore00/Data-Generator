import { DatapackFormat } from "../../util/version_util"
import { DataCategories } from "./data_categories"
import { Serializable, SerializableArray } from "./serializables"

export abstract class DataProperty<T extends Serializable> extends Serializable {
    name?: string
    optional: boolean

    constructor(category: string, name?: string) {
        super(category)
        this.name = name
        this.optional = false
    }

    abstract getValue(): T | undefined
    abstract serialize(format: DatapackFormat): any

    setOptional(): DataProperty<T> {
        this.optional = true
        return this
    }
}

export class StandardDataProperty<T extends Serializable> extends DataProperty<T> {
    value?: T

    constructor(defaultValue: T | string, name?: string) {
        super(defaultValue instanceof Serializable ? defaultValue.category : defaultValue, name)

        if (defaultValue instanceof Serializable) {
            this.value = defaultValue
        }
    }

    getValue(): T | undefined {
        return this.value
    }

    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name != "") {
            output = this.name + ":"
        }

        output += this.value?.serialize(format)
        return output
    }
}

export class ObjectDataProperty extends DataProperty<SerializableArray<DataProperty<any>>> {
    properties: DataProperty<any>[]

    constructor(name?: string) {
        super(DataCategories.NONE, name)
        this.properties = new Array<DataProperty<any>>
    }

    addProperty(property: DataProperty<any>): ObjectDataProperty {
        this.properties.push(property)
        return this
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

    constructor(name?: string) {
        super(DataCategories.PRIMITIVE.ARRAY, name)
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
    
            output += entry.serialize(format) + ",\n"
        }

        output = output.slice(0, -1) // Remove the trailing comma
        output += "]"

        return output
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