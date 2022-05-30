import { useForm } from "react-hook-form";
import { object, string, number } from 'yup';

let userSchema = object({
    name: string().required(),
    age: number().required().positive().integer(),
    address: string(),
});

export default function Form() {
    const { register, handleSubmit, watch, formState: { errors } } = useForm();
    const onSubmit = async(data: any) => {
        try {
            const user = await userSchema.validate(data);
            console.log(user);
        } catch (e) {
            console.log(e);
        }
    }

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <a>Name</a>
            <input {...register("name", { required: true })} />
            
            <a>Age</a>
            <input type="number" {...register("age", { required: true, min: 0, max: 99 })} />
            
            <a>Address</a>
            <input {...register("address")} />

            {/* errors will return when field validation fails  */}
            {errors.exampleRequired && <span>This field is required</span>}

            <input type="submit" />
        </form>
    );
}