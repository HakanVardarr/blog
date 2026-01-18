import { defineCollection, z } from "astro:content";

const blog = defineCollection({
  schema: z.object({
    title: z.string(),
    category: z.string(),
    date: z.date(),
    description: z.string(),
  }),
});

export const collections = { blog };
