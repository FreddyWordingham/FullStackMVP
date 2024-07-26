import { FC, ReactNode, useCallback, useMemo, useState } from "react";

import axios, { AxiosInstance } from "axios";

import {
  AuthenticationApi,
  CalculatorApi,
  Configuration,
  TestsApi,
  Profile,
} from "../api/my_api";

import APIContext from "../contexts/APIContext";

const APIProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [profile, setProfile] = useState<Profile | null>(null);

  const createApiClient = useCallback((): AxiosInstance => {
    const apiClient = axios.create({
      baseURL: "http://localhost:8000/api",
    });
    return apiClient;
  }, []);

  const authAPI = useMemo(
    () =>
      new AuthenticationApi(new Configuration(), undefined, createApiClient()),
    [createApiClient]
  );
  const calculatorAPI = useMemo(
    () => new CalculatorApi(new Configuration(), undefined, createApiClient()),
    [createApiClient]
  );
  const testsAPI = useMemo(
    () => new TestsApi(new Configuration(), undefined, createApiClient()),
    [createApiClient]
  );

  const state = useMemo(
    () => ({
      authAPI,
      calculatorAPI,
      testsAPI,
      profile,
    }),
    [authAPI, calculatorAPI, testsAPI, profile]
  );

  return <APIContext.Provider value={state}>{children}</APIContext.Provider>;
};

export default APIProvider;
