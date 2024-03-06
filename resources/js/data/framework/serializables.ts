import { DatapackFormat } from "../../util/version_util"

export interface Serializable {
    serialize(format: DatapackFormat): any
}

export class SerializableArray<T extends Serializable> implements Serializable {
    value: T[]

    constructor(value: T[]) {
        this.value = value
    }

    serialize(format: DatapackFormat): any {
        return this.value.toString()
    }
}

type Primitive = bigint | number | boolean | string

abstract class SerializablePrimitive<T extends Primitive> implements Serializable {
    type: PrimitiveType
    value: T

    constructor(type: PrimitiveType, value: T) {
        this.type = type
        this.value = value
    }

    serialize(format: DatapackFormat): any {
        return "" + this.value
    }
}

// Minecraft uses more primitive datatypes than javascript has,
// so there needs to be a way to distinguish between number sizes
export enum PrimitiveType {
    LONG,
    INT,
    SHORT,
    BYTE,
    
    FLOAT,

    BOOLEAN,

    RESOURCE_LOCATION
}

export enum ResourceLocationType {
    BLOCK_OR_ITEM,
    BIOME
}

export class SerializableLong extends SerializablePrimitive<bigint> {
    static of(value: bigint): SerializableLong {
        return new SerializableLong(PrimitiveType.LONG, value)
    }
}

export class SerializableInt extends SerializablePrimitive<number> {
    static of(value: number): SerializableInt {
        return new SerializableInt(PrimitiveType.INT, value)
    }
}

export class SerializableShort extends SerializablePrimitive<number> {
    static of(value: number): SerializableShort {
        return new SerializableShort(PrimitiveType.SHORT, value)
    }
}

export class SerializableByte extends SerializablePrimitive<number> {
    static of(value: number): SerializableByte {
        return new SerializableByte(PrimitiveType.BYTE, value)
    }
}

export class SerializableFloat extends SerializablePrimitive<number> {
    static of(value: number): SerializableFloat {
        return new SerializableFloat(PrimitiveType.FLOAT, value)
    }
}

export class SerializableBoolean extends SerializablePrimitive<boolean> {
    static of(value: boolean): SerializableBoolean {
        return new SerializableBoolean(PrimitiveType.BOOLEAN, value)
    }
}

export class SerializableResourceLocation extends SerializablePrimitive<string> {
    static of(value: string): SerializableResourceLocation {
        return new SerializableResourceLocation(PrimitiveType.RESOURCE_LOCATION, value)
    }
}