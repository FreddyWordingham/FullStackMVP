import { createContext } from "react";

import {
  AuthenticationApi,
  CalculatorApi,
  TestsApi,
  Profile,
} from "../api/my_api";

interface APIContextType {
  authAPI: AuthenticationApi;
  calculatorAPI: CalculatorApi;
  testsAPI: TestsApi;
  profile: Profile | null;
}

const APIContext = createContext<APIContextType | undefined>(undefined);

export default APIContext;
