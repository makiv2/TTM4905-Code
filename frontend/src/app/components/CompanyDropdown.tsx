import React from "react";
import Select, { components } from "react-select";

interface Company {
  name: string;
  logo: string;
}

interface Props {
  companies: Company[];
  selectedCompany: Company;
  onChange: (company: Company) => void;
}

const CompanyDropdown: React.FC<Props> = ({
  companies,
  selectedCompany,
  onChange,
}) => {
  const customStyles = {
    control: (styles: any) => ({
      ...styles,
      backgroundColor: "white",
      borderRadius: "0.375rem",
      borderColor: "#e2e8f0",
    }),
    option: (styles: any, { isFocused, isSelected }: any) => ({
      ...styles,
      backgroundColor: isSelected ? "#4a5568" : isFocused ? "#e2e8f0" : "white",
      color: isSelected ? "white" : "#4a5568",
    }),
    singleValue: (styles: any) => ({
      ...styles,
      color: "#000000",
    }),
    dropdownIndicator: (styles: any) => ({
      ...styles,
      color: "#000000",
    }),
    indicatorSeparator: (styles: any) => ({
      ...styles,
      backgroundColor: "#000000",
    }),
  };

  const CustomOption: React.FC<any> = (props) => (
    <components.Option {...props}>
      <div style={{ display: "flex", alignItems: "center" }}>
        <img
          src={props.data.logo}
          alt={props.data.name}
          style={{ width: 20, height: 20, marginRight: 10 }}
        />
        {props.data.name}
      </div>
    </components.Option>
  );

  const CustomSingleValue: React.FC<any> = ({ children, ...props }) => (
    <components.SingleValue {...props}>
      <div style={{ display: "flex", alignItems: "center" }}>
        <img
          src={props.data.logo}
          alt={props.data.name}
          style={{ width: 20, height: 20, marginRight: 10 }}
        />
        {children}
      </div>
    </components.SingleValue>
  );

  return (
    <Select
      options={companies}
      value={selectedCompany}
      onChange={(option: any) => onChange(option)}
      getOptionLabel={(option) => option.name}
      getOptionValue={(option) => option.name}
      styles={customStyles}
      components={{ Option: CustomOption, SingleValue: CustomSingleValue }}
      isSearchable
    />
  );
};

export default CompanyDropdown;
