import { DatapackFormat } from "../../util/version_util"
import { DataCategories } from "./data_categories"

export abstract class Serializable {
    category: string

    constructor(category: string) {
        this.category = category
    }

    abstract serialize(format: DatapackFormat): any
}

export class SerializableArray<T extends Serializable> extends Serializable {
    value?: T[]

    constructor(value?: T[]) {
        super(DataCategories.PRIMITIVE.ARRAY)
        this.value = value
    }

    serialize(format: DatapackFormat): any {
        return this.value?.toString()
    }
}

type Primitive = bigint | number | boolean | string

abstract class SerializablePrimitive<T extends Primitive> extends Serializable {
    value?: T

    constructor(category: string, value?: T) {
        super(category)
        this.value = value
    }

    serialize(format: DatapackFormat): any {
        return "" + this.value
    }
}

export class SerializableLong extends SerializablePrimitive<bigint> {
    static CATEGORY: string = DataCategories.PRIMITIVE.LONG
    constructor(value: bigint) { super(SerializableLong.CATEGORY, value) }
    static of(value: bigint): SerializableLong { return new SerializableLong(value) }
}

export class SerializableInt extends SerializablePrimitive<number> {
    static CATEGORY: string = DataCategories.PRIMITIVE.INT
    constructor(value: number) { super(SerializableInt.CATEGORY, value) }
    static of(value: number): SerializableInt { return new SerializableInt(value) }
}

export class SerializableShort extends SerializablePrimitive<number> {
    static CATEGORY: string = DataCategories.PRIMITIVE.SHORT
    constructor(value: number) { super(SerializableShort.CATEGORY, value) }
    static of(value: number): SerializableShort { return new SerializableShort(value) }
}

export class SerializableByte extends SerializablePrimitive<number> {
    static CATEGORY: string = DataCategories.PRIMITIVE.BYTE
    constructor(value: number) { super(SerializableByte.CATEGORY, value) }
    static of(value: number): SerializableByte { return new SerializableByte(value) }
}

export class SerializableFloat extends SerializablePrimitive<number> {
    static CATEGORY: string = DataCategories.PRIMITIVE.FLOAT
    constructor(value: number) { super(SerializableFloat.CATEGORY, value) }
    static of(value: number): SerializableFloat { return new SerializableFloat(value) }
}

export class SerializableDouble extends SerializablePrimitive<number> {
    static CATEGORY: string = DataCategories.PRIMITIVE.DOUBLE
    constructor(value: number) { super(SerializableDouble.CATEGORY, value) }
    static of(value: number): SerializableDouble { return new SerializableDouble(value) }
}

export class SerializableBoolean extends SerializablePrimitive<boolean> {
    static CATEGORY: string = DataCategories.PRIMITIVE.BOOLEAN
    constructor(value: boolean) { super(SerializableBoolean.CATEGORY, value) }
    static of(value: boolean): SerializableBoolean { return new SerializableBoolean(value) }
}

export class SerializableString<T extends string> extends SerializablePrimitive<T> {
    static CATEGORY: string = DataCategories.PRIMITIVE.STRING
    constructor(value: T) { super(SerializableString.CATEGORY, value) }
    static of<T extends string>(value: T): SerializableString<T> { return new SerializableString<T>(value) }
}

export class SerializableResourceLocation extends SerializablePrimitive<string> {
    static of(category: string, value: string): SerializableResourceLocation {
        return new SerializableResourceLocation(category, value)
    }
}