export interface Goal {
  id: number;
  label: string;
  description?: string;
  completed: boolean;
  faqs: Faq[];
  prerequisites: Prerequisite[];
}

interface Faq {
  id: number;
  title: string;
  content: string;
}

interface Prerequisite {
  /* type could be: "credential_definiton" (such as dif-presentation-exchange) or "goal" (other goals) or "custom" */
  type: string;
  data: string;
}
